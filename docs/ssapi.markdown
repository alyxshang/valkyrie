# VALKYRIE'S SERVER-TO-SERVER API

The API paths elaborated on below are meant for Valkyrie instances to
communicate with each other and fetch information on a user or receive
requests to carry out certain actions relating to their users. All requests 
return responses in JSON form and those that are `POST`
requests expect JSON payloads. Payloads and responses are illustrated
here as a dictionary of the payload's or responses's fields and the
types that the value should have. The required HTTP verb and the API
route's path are given in the heading for each API route. All
server-to-server API paths have the `/protocol` path prefix.


## `GET /protocol/{user}`

This API path returns information on a user. The expression `{user}` is
a placeholder for the username of an actual user. This service can be
called by anyone, including other Valkyrie instances. The shape of the
data returned by this API path is outlined below and is sent by a
Valkyrie instance in the JSON format. The response contains basic
information on the user, including the URL that any federated actions
need to be sent to (`inbox_url`) and a complete list of all the posts they have
made.

```Typescript
{
    username: string,
    display_name: string,
    description: string,
    inbox_url: string,
    post_count: number,
    posts: List<Post>
}
```

The `Post` entity has the following shape:

```Typescript
{
    post_id: string,
    created_at: string, 
    created_by: string, 
    post_content: string
}
```

## `POST /protocol/{user}/inbox`

This API path allows Valkyrie instances to talk to each other by sending
each other messages in JSON form. This endpoint ever only receives
messages from other Valkyrie instances. The expression `{user}` is
a placeholder for the username of an actual user a message is being sent
to. This service can be called by anyone, including other Valkyrie 
instances. The shape of the data returned by this API path is outlined 
below and is sent by a Valkyrie instance in the JSON format. Each
message contains information on the action that needs to be carried out
(`"Create"`, `"Update"`, `"Delete"`), the type of entity the action needs to
be carried out on (`"Post"`, `"Subscription"`), and the object containing 
information on the actual entity. Field names that have the
`host` suffix in the name, give information on which user on which
instance carried out an action and on which entity the action has been
carried out. The shape of the message, sent in JSON
form, is outlined below:

```Typescript
{
    action: Action,
    entity: Entity,
    object: Object
}
```

The `Object` can be of two types: either a `PostObject` or a `SubscriberObject`.
The `PostObject` has the shape outlined below. This object can be
created, updated or deleted.

```Typescript
{
    post_id: string,
    created_at: string, 
    created_by: string,
    created_by_host: string,
    post_content: string
}
```

The `SubscriberObject` has the shape outlined below. These objects can
only be deleted or created. The ID field contains the ID of the 
following-followee relationship created on the
server of the user who initiated the relationship:

```Typescript
{
    id: string,
    sender_host: string,
    sender_handle: string,
    receiver_host: string,
    receiver_handle: string
}
```

The boolean in the response indicates whether the attempted operation
was successful or not. The response of this API path has the shape as 
illustrated below:

```Typescript
{
    is_ok: bool
}
```
