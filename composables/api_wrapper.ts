import {
    type apistorageservicesv2DeleteObjectResponse,
    type modelsv2License,
    type v2Collection,
    type v2CreateAPITokenRequest,
    type v2CreateAPITokenResponse,
    type v2CreateCollectionRequest,
    type v2CreateCollectionResponse,
    type v2CreateDatasetRequest,
    type v2CreateDatasetResponse,
    type v2CreateObjectRequest,
    type v2CreateObjectResponse,
    type v2CreateProjectRequest,
    type v2CreateProjectResponse,
    type v2CreateS3CredentialsUserTokenResponse,
    type v2Dataset,
    type v2Endpoint,
    type v2GetDownloadURLResponse,
    type v2GetHierarchyResponse,
    type v2GetResourceResponse,
    type v2GetS3CredentialsUserTokenResponse,
    type v2GetUploadURLResponse,
    v2InternalRelationVariant,
    type v2Object,
    type v2Permission,
    type v2Project,
    v2RelationDirection,
    v2ResourceVariant,
    type v2ResourceWithPermission,
    type v2SearchResourcesResponse,
    type v2User
} from "./aruna_api_json"
import {type ObjectInfo, toObjectInfo} from "~/composables/proto_conversions";
import type {ArunaError} from "~/composables/ArunaError";

export async function searchResources(query: string): Promise<v2SearchResourcesResponse> {
    return await $fetch<v2SearchResourcesResponse>('api/search', {
        method: 'POST',
        body: query
    }).catch(error => {
        console.error(error)
        throw new Error("Resource search failed.")
    })
}

export async function fetchEndpoints(): Promise<v2Endpoint[] | undefined> {
    // Fetch endpoints
    return await $fetch('/api/endpoints').catch(error => {
        console.error(error)
        return []
    })
}

export async function fetchEndpoint(endpointId: string): Promise<v2Endpoint | undefined> {
    // Fetch endpoints
    return await $fetch<v2Endpoint>('/api/endpoint', {
        method: 'GET',
        query: {
            endpointId: endpointId
        }
    }).catch(error => {
        console.error(error)
        throw Error("Failed to fetch endpoint")
    })
}

export async function fetchLicenses(): Promise<modelsv2License[] | undefined> {
    // Fetch licenses
    const licenses = await $fetch<modelsv2License[]>('/api/licenses')
    return licenses
}

export async function fetchUser(id: string | undefined): Promise<v2User | string> {
    const user = await $fetch(id ? `/api/user?userId=${id}` : '/api/user').catch((e) => {
        return e.toString()
    })
    return user
}

export async function fetchUsers(): Promise<v2User[] | undefined> {
    const users = await $fetch('/api/users')
    return users
}

export async function activateUser(userId: string): Promise<boolean> {
    const response = await $fetch(`/api/user/${userId}/activate`, {
        method: 'PATCH'
    })
    return response !== undefined
}

export async function deactivateUser(userId: string): Promise<boolean> {
    // Deactivate user
    const response = await $fetch(`/api/user/${userId}/deactivate`, {
        method: 'PATCH'
    })
    return response !== undefined
}

export async function createUserToken(
    name: string,
    scope: v2Permission | undefined,
    expiry: string | undefined): Promise<v2CreateAPITokenResponse | undefined> {
    // Create request and send
    const request = {
        name: name,
        permission: scope,
        expiresAt: expiry
    } as v2CreateAPITokenRequest

    return await $fetch<v2CreateAPITokenResponse>('/api/user/tokens', {
        method: 'POST',
        body: request
    })
}

export async function createUserS3Credentials(endpointId: string): Promise<v2CreateS3CredentialsUserTokenResponse> {
    return await $fetch<v2CreateS3CredentialsUserTokenResponse>(`/api/user/s3_credentials/${endpointId}`, {
        method: 'PATCH',
    }).catch(error => {
        throw Error("Failed to create S3 credentials. Please try again later.")
    })
}

export async function getUserS3Credentials(endpointId: string): Promise<v2GetS3CredentialsUserTokenResponse> {
    return await $fetch<v2GetS3CredentialsUserTokenResponse>(`/api/user/s3_credentials/${endpointId}`, {
        method: 'GET',
    }).catch(error => {
        throw Error("Failed to fetch S3 credentials. Please try again later.")
    })
}

export async function fetchResource(resourceId: string): Promise<v2ResourceWithPermission | undefined> {
    return $fetch<v2GetResourceResponse>('/api/resource', {
        method: 'GET',
        query: {
            resourceId: resourceId
        }
    }).then(response => {
        return response.resource
    }).catch(error => {
        if (error.message.includes('404')) {
            return undefined // Not found
        }
        // Bubble every other error up
        throw new Error("Failed to fetch resource")
    })
}

export async function fetchUserResources(user: v2User | undefined): Promise<v2ResourceWithPermission[]> {
    if (user === undefined) {
        return []
    }

    // Filter projects from user permissions
    const projectPermissions = user.attributes?.personalPermissions?.filter((perm) => perm.projectId)
    // Fetch resources
    if (projectPermissions && projectPermissions.length > 0) {
        let fetchUrl = '/api/resources?'
        projectPermissions.forEach((perm, idx, arr) => {
            if (perm.projectId) {
                fetchUrl += arr.length - 1 === idx ? `resourceIds=${perm.projectId}` : `resourceIds=${perm.projectId}&`
            }
        })

        const resources = await $fetch<v2ResourceWithPermission[]>(fetchUrl)
        return resources
    }

    return []
}

export async function createProject(request: v2CreateProjectRequest): Promise<v2Project | undefined> {
    return $fetch<v2CreateProjectResponse>('/api/project', {
        method: 'POST',
        body: request
    }).then(response => {
        return response.project
    }).catch(error => {
        console.error(error)
        throw new Error("Project creation failed.")
    })
}

export async function createCollection(request: v2CreateCollectionRequest): Promise<v2Collection | undefined> {
    return $fetch<v2CreateCollectionResponse>('/api/collection', {
        method: 'POST',
        body: request
    }).then(response => {
        return response.collection
    }).catch(error => {
        console.error(error)
        throw new Error("Collection creation failed.")
    })
}

export async function createDataset(request: v2CreateDatasetRequest): Promise<v2Dataset | undefined> {
    return $fetch<v2CreateDatasetResponse>('/api/dataset', {
        method: 'POST',
        body: request
    }).then(response => {
        return response.dataset
    }).catch(error => {
        console.error(error)
        throw new Error("Dataset creation failed.")
    })
}

export async function createObject(request: v2CreateObjectRequest): Promise<v2Object | undefined> {
    return $fetch<v2CreateObjectResponse>('/api/object', {
        method: 'POST',
        body: request
    }).then(response => {
        return response.object
    }).catch(error => {
        console.error(error)
        throw new Error("Object creation failed.")
    })
}

export async function deleteObject(objectId: string, withRevisions: boolean): Promise<boolean> {
    return $fetch<apistorageservicesv2DeleteObjectResponse>('/api/object', {
        method: 'DELETE',
        query: {
            objectId: objectId
        },
        body: {
            withRevisions: withRevisions
        }
    }).then(response => {
        return typeof response !== 'undefined'
    }).catch(error => {
        console.error(error)
        throw new Error("Object deletion failed.")
    })
}

export async function getUploadUrl(resourceId: string) {
    return $fetch<v2GetUploadURLResponse | ArunaError>(`/api/object/${resourceId}/upload`, {
        method: 'GET'
    }).catch(error => {
        throw Error('Failed to fetch resource upload url. Please try again later')
    })
}

export async function getDownloadUrl(resourceId: string) {
    return $fetch<v2GetDownloadURLResponse>(`/api/object/${resourceId}/download`, {
        method: 'GET'
    }).catch(error => {
        console.error(error)
        throw Error("Failed to fetch resource download url. Please try again later.")
    })
}

export async function getResourceHierarchy(resourceId: string) {
    return $fetch<v2GetHierarchyResponse>(`/api/relation/${resourceId}/hierarchy`, {
        method: 'GET'
    }).catch(error => {
        console.error(error)
        throw Error("Failed to fetch resource hierarchy. Please try again later.")
    })
}

export async function getPublicResourceUrl(endpointHost: string, resource: ObjectInfo, ssl: boolean): Promise<string> {
    // Hierarchy objects are downloaded through the special Objects bucket
    if (resource.variant !== v2ResourceVariant.RESOURCE_VARIANT_OBJECT) {
        return ssl ?
            `https://objects.${endpointHost}/${resource.id}/${resource.name}.tar.gz` :
            `http://objects.${endpointHost}/${resource.id}/${resource.name}.tar.gz`
    }

    // Else traverse hierarchy up
    let project = undefined
    let collection = undefined
    let dataset = undefined
    let object = resource.name

    while (resource.variant !== v2ResourceVariant.RESOURCE_VARIANT_PROJECT) {
        const parentRel = resource.relations.find(rel =>
            rel.internal?.direction === v2RelationDirection.RELATION_DIRECTION_INBOUND &&
            rel.internal?.definedVariant === v2InternalRelationVariant.INTERNAL_RELATION_VARIANT_BELONGS_TO
        )

        if (parentRel?.internal?.resourceId) {
            const parentObj = await fetchResource(parentRel.internal.resourceId)
            if (parentObj) {
                const objectInfo = toObjectInfo(parentObj.resource, parentObj.permission)
                if (objectInfo) {
                    switch (objectInfo.variant) {
                        case v2ResourceVariant.RESOURCE_VARIANT_PROJECT: {
                            project = objectInfo.name
                            break
                        }
                        case v2ResourceVariant.RESOURCE_VARIANT_COLLECTION: {
                            collection = objectInfo.name
                            break
                        }
                        case v2ResourceVariant.RESOURCE_VARIANT_DATASET: {
                            dataset = objectInfo.name
                            break
                        }
                    }
                    resource = objectInfo
                } else {
                    throw Error("Conversion to ObjectInfo failed")
                }
            } else {
                throw Error(`Parent resource (${parentRel.internal.resourceId}) does not exist`)
            }
        } else {
            throw Error(`Resource (${resource.id}:${resource.name}) has no parent relations`)
        }
    }

    // Build url from its parts
    const schema = ssl ? 'https' : 'http'
    let url = `${schema}://${project}.${endpointHost}`
    url += collection ? `/${collection}` : ''
    url += dataset ? `/${dataset}` : ''
    url += `/${object}`

    return url
}