use crate::utils::structs::ResourceType;
use aruna_rust_api::api::storage::models::v2::License;
use leptos::{html::Input, *};
use leptos_router::ActionForm;

#[server(CreateResourceRequest)]
pub async fn create_resource_request(
    #[server(default)] resname: String,
    #[server(default)] resource_type: String,
    #[server(default)] description: String,
    #[server(default)] dataclass: i32,
    #[server(default)] metadata_license: String,
    #[server(default)] data_license: String,
    #[server(default)] parent: Option<String>,
    #[server(default)] parent_type: Option<ResourceType>,
) -> Result<(), ServerFnError> {
    use crate::utils::aruna_api_handlers::ConnectionHandler;
    use axum_extra::extract::CookieJar;
    use leptos::logging::log;
    use leptos_axum::redirect;

    let req_parts = use_context::<leptos_axum::RequestParts>()
        .ok_or_else(|| ServerFnError::Request("Invalid context".to_string()))?;
    let jar = CookieJar::from_headers(&req_parts.headers);

    let token = if let Some(cookie) = jar.get("token") {
        cookie.value().to_string()
    } else {
        return Err(ServerFnError::Args(
            "Custom error: Token not found".to_string(),
        ));
    };
    let res_type = match resource_type.as_str() {
        "Project" => ResourceType::Project,
        "Collection" => ResourceType::Collection,
        "Dataset" => ResourceType::Dataset,
        "Object" => ResourceType::Object,
        _ => {
            return Err(ServerFnError::ServerError(
                "Invalid resource type".to_string(),
            ))
        }
    };
    let res = ConnectionHandler::aruna_create_resource(
        &token,
        &resname,
        &description,
        res_type,
        data_license,
        metadata_license,
        parent,
        dataclass,
        parent_type,
    )
    .await;
    log!("{res:?}");
    if res.is_ok() {
        redirect("/objects");
    } else {
        return Err(ServerFnError::ServerError(
            "CreateResource failed.".to_string(),
        ));
    }
    Ok(())
}

#[server]
pub async fn get_licenses() -> Result<Vec<License>, ServerFnError> {
    use crate::utils::aruna_api_handlers::ConnectionHandler;
    use axum_extra::extract::CookieJar;
    use leptos::logging::log;

    let req_parts = use_context::<leptos_axum::RequestParts>()
        .ok_or_else(|| ServerFnError::Request("Invalid context".to_string()))?;
    let jar = CookieJar::from_headers(&req_parts.headers);

    let token = if let Some(cookie) = jar.get("token") {
        cookie.value().to_string()
    } else {
        return Err(ServerFnError::Args(
            "Custom error: Token not found".to_string(),
        ));
    };

    let res = ConnectionHandler::get_licenses(&token).await;
    log!("{res:?}");
    if res.is_ok() {
        Ok(res.unwrap())
    } else {
        return Err(ServerFnError::ServerError(
            "GetLicenses failed.".to_string(),
        ));
    }
}

#[component]
pub fn CreateObjectPage() -> impl IntoView {
    let create_resource_action = create_server_action::<CreateResourceRequest>();
    let get_licenses_result = create_local_resource(
        || (),
        move |_| async move {
            let result = get_licenses().await;
            result.unwrap_or_default()
        },
    );
    let _succeeded = move || {
        create_resource_action.value()()
            .map(|e| e.ok())
            .flatten()
            .is_some()
    };
    let header = move || {
        view! {
            <div class="container-xl">
                <div class="row g-2">
                    <div class="col">
                        <div class="page-pretitle text-start">
                            Create
                        </div>
                        <h2 class="page-title">
                            Resource
                        </h2>
                    </div>
                    <div class="col-auto ms-auto d-print-none text-end">
                        <div class="btn-list">
                            <a
                                onclick="history.back()"
                                class="btn btn-ghost-secondary d-none d-sm-inline-block pe-0 ps-3"
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="icon icon-tabler icon-tabler-arrow-left"
                                    width="40"
                                    height="40"
                                    viewBox="0 0 24 24"
                                    stroke-width="2"
                                    stroke="currentColor"
                                    fill="none"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                >
                                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                    <path d="M5 12l14 0"></path>
                                    <path d="M5 12l6 6"></path>
                                    <path d="M5 12l6 -6"></path>
                                </svg>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        }
    };

    let (read_type_select, write_type_select) = create_signal::<String>("Project".to_string());
    let (_file_size, write_file_size) = create_signal::<Option<u64>>(None);

    let input_element = create_node_ref::<Input>();

    let _on_file_change = move |_ev: leptos::ev::Event| {
        if let Some(files) = input_element.get().map(|fi| fi.files()).flatten() {
            let file = files.get(0).unwrap();
            write_file_size(Some(file.size() as u64));
            // let file_blob_promise = js_sys::Promise::resolve(&file.array_buffer());
            // //set_file_name(Some(file.name()));
            // spawn_local(async move {
            //     let bytes = wasm_bindgen_futures::JsFuture::from(file_blob_promise)
            //         .await
            //         .unwrap();
            //     let byte_arr = js_sys::Uint8Array::new(&bytes);
            //     //let _ = SaveFile::from_bytes_rs(buffer);
            //     // let sf = SaveFile::from_bytes_rs(buffer).unwrap();
            //     let client = reqwest::Client::new();
            //     let resp = client
            //         .put("http://localhost:8000")
            //         .body(byte_arr.to_vec())
            //         .send()
            //         .await
            //         .unwrap();
            //     leptos::log!("{:?}", resp);
            // })
        }
    };

    let main = move || {
        view! {
            <div class="row row-cards">
                <div class="col-lg-12">
                    //<form class="card">
                    <ActionForm
                        class="card"
                        action=create_resource_action
                    >
                        <div class="card-body">
                            <div class="row g-5">
                                <div class="col-lg-4 mb-3 text-start">
                                    <label class="form-label">"Resource type"</label>
                                    <div class="form-selectgroup">
                                        <label class="form-selectgroup-item">
                                            <input
                                                type="radio"
                                                name="resource_type"
                                                value="Project"
                                                class="form-selectgroup-input"
                                                checked=""
                                                on:input=move |ev| {
                                                    write_type_select(event_target_value(&ev));
                                                }
                                            />
                                            <span class="form-selectgroup-label">
                                                Project
                                            </span>
                                        </label>
                                        <label class="form-selectgroup-item">
                                            <input
                                                type="radio"
                                                name="resource_type"
                                                value="Collection"
                                                class="form-selectgroup-input"
                                                on:input=move |ev| {
                                                    write_type_select(event_target_value(&ev));
                                                }
                                            />
                                            <span class="form-selectgroup-label">
                                                Collection
                                            </span>
                                        </label>
                                        <label class="form-selectgroup-item">
                                            <input
                                                type="radio"
                                                name="resource_type"
                                                value="Dataset"
                                                class="form-selectgroup-input"
                                                on:input=move |ev| {
                                                    write_type_select(event_target_value(&ev));
                                                }
                                            />
                                            <span class="form-selectgroup-label">
                                                Dataset
                                            </span>
                                        </label>
                                        <label class="form-selectgroup-item">
                                            <input
                                                type="radio"
                                                name="resource_type"
                                                value="Object"
                                                class="form-selectgroup-input"
                                                on:input=move |ev| {
                                                    write_type_select(event_target_value(&ev));
                                                }
                                            />
                                            <span class="form-selectgroup-label">
                                                Object
                                            </span>
                                        </label>
                                    </div>
                                    <small class="form-hint">"Every resource has a type"</small>
                                </div>
                                <div class="col-lg-4 mb-3 text-start">
                                    <label class="form-label">"Visibility class"</label>
                                    <div class="form-selectgroup">
                                        <label class="form-selectgroup-item">
                                            <input
                                                type="radio"
                                                name="dataclass"
                                                value=1
                                                class="form-selectgroup-input"
                                                checked=""
                                            />
                                            <span class="form-selectgroup-label">
                                                Public
                                            </span>
                                        </label>
                                        <label class="form-selectgroup-item">
                                            <input
                                                type="radio"
                                                name="dataclass"
                                                value=2
                                                class="form-selectgroup-input"
                                            />
                                            <span class="form-selectgroup-label">
                                                Private
                                            </span>
                                        </label>
                                        <label class="form-selectgroup-item">
                                            <input
                                                type="radio"
                                                name="dataclass"
                                                value=4
                                                class="form-selectgroup-input"
                                            />
                                            <span class="form-selectgroup-label">
                                                Workspace
                                            </span>
                                        </label>
                                        <label class="form-selectgroup-item">
                                            <input
                                                type="radio"
                                                name="dataclass"
                                                value=5
                                                class="form-selectgroup-input"
                                            />
                                            <span class="form-selectgroup-label">
                                                Confidential
                                            </span>
                                        </label>
                                    </div>
                                    <small class="form-hint">
                                        "Every resource has a data class"
                                    </small>
                                </div>
                                <div class="col-lg-4 mb-3 text-start">
                                    <label class="form-label">"Licenses"</label>
                                    <Transition
                                        fallback=move || view!{ <p>"Loading licenses ..." </p>}
                                    >
                                        {move ||
                                            {
                                                let licenses = get_licenses_result.get().unwrap_or_default();
                                                let licenses_copy = licenses.clone();
                                                view!{
                                                    <select class="form-select" name="metadata_license">
                                                        <For
                                                        each=move || licenses.clone()
                                                        key=|result| {
                                                            result.tag.clone()
                                                        }
                                                        children=move |res| {
                                                            view! { <option value={res.tag.clone()}>{res.tag.clone()}</option> }
                                                        }
                                                        />
                                                    </select>
                                                    <small class="form-hint">
                                                        "Metadata license"
                                                    </small>
                                                    <select class="form-select" name="data_license">
                                                        <For
                                                        each=move || licenses_copy.clone()
                                                        key=|result| {
                                                            result.tag.clone()
                                                        }
                                                        children=move |res| {
                                                            view! { <option value={res.tag.clone()}>{res.tag.clone()}</option> }
                                                        }
                                                        />
                                                    </select>
                                                    <small class="form-hint">
                                                        "Data license (default)"
                                                    </small>
                                                }
                                            }
                                        }
                                    </Transition>

                                </div>
                                <div class="col-lg-4 mb-3 text-start">
                                    <label class="form-label required">"Name"</label>
                                    <div>
                                        <input
                                            type="text"
                                            class="form-control text-uppercase"
                                            id="resname"
                                            name="resname"
                                            placeholder="Resource Name"
                                            required
                                        />
                                        <small class="form-hint">
                                            "The name of the resource, cannot contain spaces or special characters"
                                        </small>
                                    </div>
                                </div>

                                <Show when=move || read_type_select() != "Project" fallback=|| ()>
                                    <div class="col-lg-4 mb-3 text-start">
                                        <label class="form-label required">"Parent type"</label>
                                        {move || match read_type_select().as_str() {
                                            "Collection" => view!{
                                                <div class="form-selectgroup">
                                                    <label class="form-selectgroup-item">
                                                        <input
                                                            type="radio"
                                                            name="parent_type"
                                                            value="Project"
                                                            class="form-selectgroup-input"
                                                            checked=""
                                                        />
                                                        <span class="form-selectgroup-label">
                                                            Project
                                                        </span>
                                                    </label>
                                                </div>
                                                <small class="form-hint">
                                                    "Collections can only have projects as parents"
                                                    <p/>
                                                </small>
                                            }.into_view(),
                                            "Dataset" => view!{
                                                <div class="form-selectgroup">
                                                    <label class="form-selectgroup-item">
                                                        <input
                                                            type="radio"
                                                            name="parent_type"
                                                            value="Project"
                                                            class="form-selectgroup-input"
                                                            checked=""
                                                        />
                                                        <span class="form-selectgroup-label">
                                                            Project
                                                        </span>
                                                    </label>
                                                    <label class="form-selectgroup-item">
                                                        <input
                                                            type="radio"
                                                            name="parent_type"
                                                            value="Collection"
                                                            class="form-selectgroup-input"
                                                            checked=""
                                                        />
                                                        <span class="form-selectgroup-label">
                                                            Collection
                                                        </span>
                                                    </label>
                                                </div>
                                                <small class="form-hint">
                                                    "Dataset parents can only have projects and collections as parents"
                                                    <p/>
                                                </small>
                                            }.into_view(),
                                            "Object" => view!{
                                                <div class="form-selectgroup">
                                                    <label class="form-selectgroup-item">
                                                        <input
                                                            type="radio"
                                                            name="parent_type"
                                                            value="Project"
                                                            class="form-selectgroup-input"
                                                            checked=""
                                                        />
                                                        <span class="form-selectgroup-label">
                                                            Project
                                                        </span>
                                                    </label>
                                                    <label class="form-selectgroup-item">
                                                        <input
                                                            type="radio"
                                                            name="parent_type"
                                                            value="Collection"
                                                            class="form-selectgroup-input"
                                                        />
                                                        <span class="form-selectgroup-label">
                                                            Collection
                                                        </span>
                                                    </label>
                                                    <label class="form-selectgroup-item">
                                                        <input
                                                            type="radio"
                                                            name="parent_type"
                                                            value="Dataset"
                                                            class="form-selectgroup-input"
                                                     />
                                                        <span class="form-selectgroup-label">
                                                            Dataset
                                                        </span>
                                                    </label>
                                                </div>
                                                <small class="form-hint">
                                                    "Object parents can only have projects, collections or datasets as parents"
                                                    <p/>
                                                </small>
                                            }.into_view(),
                                            _ => ().into_view()
                                        }}
                                        <label class="form-label required">"Parent ID"</label>
                                        <div>
                                            <input
                                                type="text"
                                                class="form-control text-uppercase"
                                                pattern={"^[0-7][0-9A-HJKMNP-TV-Z]{25}$"}
                                                id="resid"
                                                name="parent"
                                                placeholder="Resource ID"
                                                required
                                            />
                                            <small class="form-hint">
                                                "Each non project resource must be associated with an existing parent"
                                            </small>
                                        </div>
                                    </div>
                                </Show>

                                <div class="col-12 text-start">
                                    <label class="form-label">"Description"</label>
                                    <textarea
                                        class="form-control"
                                        name="description"
                                        rows="6"
                                        placeholder="Description"
                                    ></textarea>
                                </div>

                                // <div class="table-responsive col-lg-6 mt-5 mb-3 text-start">
                                //     <label class="form-label">"Internal relations"</label>
                                //     <table class="table table-vcenter text-start">
                                //         <thead>
                                //             <tr>
                                //                 <th class="text-start">
                                //                     TARGET
                                //                 </th>
                                //                 <th>
                                //                     DIRECTION
                                //                 </th>
                                //                 <th>
                                //                     TYPE
                                //                 </th>
                                //                 <th></th>
                                //             </tr>
                                //         </thead>
                                //         <tbody>
                                //             <tr>
                                //                 <td>
                                //                     <input
                                //                         type="text"
                                //                         class="form-control"
                                //                         name="relations"
                                //                         placeholder="Target ULID"
                                //                     />
                                //                 </td>
                                //                 <td>
                                //                     <select class="form-select">
                                //                         <option value="Inc" selected="">
                                //                             Outgoing
                                //                         </option>
                                //                         <option value="Out">
                                //                             Incoming
                                //                         </option>
                                //                     </select>
                                //                 </td>
                                //                 <td>
                                //                     <select class="form-select">
                                //                         <option value="belo" selected="">
                                //                             BelongsTo
                                //                         </option>
                                //                         <option value="meta">
                                //                             Metadata
                                //                         </option>
                                //                         <option value="orig">
                                //                             Origin
                                //                         </option>
                                //                     </select>
                                //                 </td>
                                //                 <td>
                                //                     <button class="btn btn-success w-100 btn-icon">
                                //                         <svg
                                //                             xmlns="http://www.w3.org/2000/svg"
                                //                             class="icon icon-tabler icon-tabler-circle-plus"
                                //                             width="40"
                                //                             height="40"
                                //                             viewBox="0 0 24 24"
                                //                             stroke-width="2"
                                //                             stroke="currentColor"
                                //                             fill="none"
                                //                             stroke-linecap="round"
                                //                             stroke-linejoin="round"
                                //                         >
                                //                             <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                //                             <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0"></path>
                                //                             <path d="M9 12h6"></path>
                                //                             <path d="M12 9v6"></path>
                                //                         </svg>
                                //                     </button>
                                //                 </td>
                                //             </tr>
                                //         </tbody>
                                //     </table>
                                //     <small class="form-hint">
                                //         "Internal relations are connections to other resources within the system"
                                //     </small>
                                // </div>
                                // <div class="table-responsive col-lg-6 mt-5 mb-3 text-start">
                                //     <label class="form-label">"External relations"</label>
                                //     <table class="table table-vcenter text-start">
                                //         <thead>
                                //             <tr>
                                //                 <th class="text-start">
                                //                     TARGET
                                //                 </th>
                                //                 <th>
                                //                     TYPE
                                //                 </th>
                                //                 <th></th>
                                //             </tr>
                                //         </thead>

                                //         <tbody>
                                //             <tr>
                                //                 <td>
                                //                     <input
                                //                         type="text"
                                //                         class="form-control"
                                //                         placeholder="Target Identifier (URL etc.)"
                                //                     />
                                //                 </td>
                                //                 <td>
                                //                     <select class="form-select">
                                //                         <option value="url" selected="">
                                //                             URL
                                //                         </option>
                                //                         <option value="id">
                                //                             IDENTIFIER
                                //                         </option>
                                //                         <option value="doi">
                                //                             DOI
                                //                         </option>
                                //                     </select>
                                //                 </td>
                                //                 <td>
                                //                     <button class="btn btn-success w-100 btn-icon">
                                //                         <svg
                                //                             xmlns="http://www.w3.org/2000/svg"
                                //                             class="icon icon-tabler icon-tabler-circle-plus"
                                //                             width="40"
                                //                             height="40"
                                //                             viewBox="0 0 24 24"
                                //                             stroke-width="2"
                                //                             stroke="currentColor"
                                //                             fill="none"
                                //                             stroke-linecap="round"
                                //                             stroke-linejoin="round"
                                //                         >
                                //                             <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                //                             <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0"></path>
                                //                             <path d="M9 12h6"></path>
                                //                             <path d="M12 9v6"></path>
                                //                         </svg>
                                //                     </button>
                                //                 </td>
                                //             </tr>
                                //         </tbody>
                                //     </table>
                                //     <small class="form-hint">
                                //         "External relations are connections to external resources"
                                //     </small>
                                // </div>
                                // <div class="table-responsive col-12 mt-5 mb-3 text-start">
                                //     <label class="form-label">"Labels & Hooks"</label>
                                //     <table class="table table-vcenter text-start">
                                //         <thead>
                                //             <tr>
                                //                 <th class="text-start">
                                //                     Key
                                //                 </th>
                                //                 <th>
                                //                     Value
                                //                 </th>
                                //                 <th>
                                //                     TYPE
                                //                 </th>
                                //                 <th></th>
                                //             </tr>
                                //         </thead>

                                //         <tbody>
                                //             <tr>
                                //                 <td>
                                //                     <input
                                //                         type="text"
                                //                         class="form-control"
                                //                         placeholder="Key"
                                //                     />
                                //                 </td>
                                //                 <td>
                                //                     <input
                                //                         type="text"
                                //                         class="form-control"
                                //                         placeholder="Value"
                                //                     />
                                //                 </td>
                                //                 <td>
                                //                     // { custom_select("label-type".to_string(), vec!["Label".to_string(), "Hook".to_string(), "Static Label".to_string()]) }
                                //                     <select class="form-select">
                                //                         <option value="lbl" selected="">
                                //                             Label
                                //                         </option>
                                //                         <option value="hook">
                                //                             Hook
                                //                         </option>
                                //                         <option value="slbl">
                                //                             Static Label
                                //                         </option>
                                //                     </select>
                                //                 </td>
                                //                 <td colspan="1">
                                //                     <button class="btn btn-success w-100 btn-icon">
                                //                         <svg
                                //                             xmlns="http://www.w3.org/2000/svg"
                                //                             class="icon icon-tabler icon-tabler-circle-plus"
                                //                             width="40"
                                //                             height="40"
                                //                             viewBox="0 0 24 24"
                                //                             stroke-width="2"
                                //                             stroke="currentColor"
                                //                             fill="none"
                                //                             stroke-linecap="round"
                                //                             stroke-linejoin="round"
                                //                         >
                                //                             <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                //                             <path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0"></path>
                                //                             <path d="M9 12h6"></path>
                                //                             <path d="M12 9v6"></path>
                                //                         </svg>
                                //                     </button>
                                //                 </td>
                                //             </tr>
                                //         </tbody>
                                //     </table>
                                //     <small class="form-hint">
                                //         "Labels are tags that make your resource finable, hooks are actions that can be performed on the resource"
                                //     </small>
                                // </div>
                                // <Show when=move || read_type_select() == "Object" fallback=|| ()>
                                //     <div class="col-lg-4 mb-3 text-start">
                                //         <div class="mb-3">
                                //             <div class="form-label">
                                //                 Upload file
                                //             </div>
                                //             <input
                                //                 type="file"
                                //                 on:change=on_file_change
                                //                 node_ref=input_element
                                //                 class="form-control"
                                //             />
                                //         </div>
                                //         <Show when=move || file_size().is_some() fallback=|| ()>
                                //             <div class="h2 mb-0 text-muted">
                                //                 {bytesize::ByteSize(file_size().unwrap())
                                //                     .to_string_as(true)}
                                //             </div>
                                //         </Show>
                                //     </div>
                                // </Show>
                            </div>
                        </div>
                        <div class="card-footer text-end">
                            <button type="submit" class="btn btn-primary">
                                Submit
                            </button>
                        </div>
                    //</form>
                    </ActionForm>
                </div>
            </div>
        }
    };

    view! {
        <div class="page-wrapper d-print-none">
            <div class="page-header">{header}</div>
            <div class="page-body mt-2">
                <div class="container-xl">{main}</div>
            </div>
        </div>
    }
}
