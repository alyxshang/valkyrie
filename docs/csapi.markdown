# VALKYRIE'S CLIENT-TO-SERVER API

The following section explains which client-to-server API paths this
implementation of the Valkyrie server has, which paths these API routes
are addressable at, the payloads they expect, and the responses they send 
back. All requests return responses in JSON form and those that are `POST`
requests expect JSON payloads. Payloads and responses are illustrated
here as a dictionary of the payload's or responses's fields and the
types that the value should have. The required HTTP verb and the API
route's path are given in the heading for each API route. All
client-to-server API paths have the `/api` path prefix.

## `GET /api/instance`

This API path returns information on the current Valkyrie instance.
The response of this API path has the shape as illustrated below:

```Typescript
{
    name: string,
    admin: string,
    description: string
}
```

## `POST /api/instance/edit/name`

This API path allows the sole administrator of the Valkyrie instance to
edit the instance's name. The payload of this API path must have the 
shape illustrated below. The token must exist first by calling 
the API route to login the instance's administrator.

```Typescript
{
    token: string,
    new_value: string
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

## `POST /api/instance/edit/description`

This API path allows the sole administrator of the Valkyrie instance to
edit the instance's description. The payload of this API path must have the 
shape illustrated below. The token must exist first by calling 
the API route to login the instance's administrator.

```Typescript
{
    token: string,
    new_value: string
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

## `POST /api/user/create`

This API path allows anyone to create an account on a Valkyrie instance.
The payload of this API path must have the shape illustrated below:

```Typescript
{
    username: string,
    password: string,
    display_name: string,
    description: string
}
```

If the operation succeeds, public information on the user created
is returned. The response of this API path has the shape as 
illustrated below:

```Typescript
{
    username: string,
    display_name: string,
    description: string
}
```

## `POST /api/user/login`

This API path allows existing users on the current instance to login to
the instance and create an API token for their account. This token is
needed to carry out operations as a user. The payload of this API path 
must have the shape illustrated below. This API path can also be used to 
obtain an API token to carry out further operations for a user.

```Typescript
{
    username: string,
    password: string
}
```

The response returns the API token in the value of the `token` field.
The response of this API path has the shape as illustrated below:

```Typescript
{
    token: string
}
```

## `POST /api/user/logout`

This API path lets a user logout or delete their API token. The token 
must exist first by calling  the API route to login user.
The payload of this API path must have the shape illustrated below:

```Typescript
{
    token: string
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

## `POST /api/user/edit/name`

This API path allows a user to edit their display name.
The payload of this API path must have the shape illustrated below. An
API token must exist first to carry out changes for a user.

```Typescript
{
    token: string,
    new_value: string
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

## `POST /api/user/edit/description`

This API path allows a user to edit their profile description.
The payload of this API path must have the shape illustrated below. An
API token must exist first to carry out changes for a user.

```Typescript
{
    token: string,
    new_value: string
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

## `POST /api/user/edit/password`

This API path allows a user to edit their current password. Users
must supply their current password for security purposes and to prevent
accidental password changes. The payload of this API path must have the 
shape illustrated below. An API token must exist first to carry 
out changes for a user.

```Typescript
{
    token: string,
    password: string,
    new_value: string
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

## `POST /api/post/create`

This API path allows a user to create a new post. If the user is
followed by users on other instances, this post is propagated across
the network of Valkyrie instances. The payload of this API path must 
have the shape illustrated below. An API token must exist first 
to post something.

```Typescript
{
    token: string,
    content: string
}
```

The response features salient information on the post created and
returns this information in the response. The response of this API 
path has the shape as illustrated below:

```Typescript
{
    post_id: string,
    created_at: string, 
    created_by: string, 
    post_content: string
}
```

## `POST /api/post/delete`

This API path allows users to delete a post. If the user is followed
by other users on other Valkyrie instances, the deletion of this post
by this user is propagated across the network of other Valkyrie
instances. The payload of this API path must have the shape illustrated 
below. An API token must exist first to delete a post.

```Typescript
{
    token: string,
    post_id: string
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

## `POST /api/post/update`

This API path allows users to update a post they have created. If the 
user is followed by other users on other Valkyrie instances, the
updating of this post by this user is propagated across the network of
other Valkyrie instances. The payload of this API path must have the 
shape illustrated below. An API token must exist first to update 
a post.

```Typescript
{
    token: string,
    content: string,
    post_id: string
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

## `POST /api/user/tokens/list`

This API path allows users to get a list of all the API tokens that are
currently active and being used. The payload of this API path must 
have the shape illustrated below. An API token must exist first 
to view the list of active API tokens.

```Typescript
{
    token: string,
}
```

The tokens in this response are returned as a list of strings. This
list is the value of the `tokens` field. The response of this API 
path has the shape as illustrated below:

```Typescript
{
    tokens: List<string>
}
```

## `POST /api/user/follow`

This API path allows users to follow users on their own instance or
other Valkyrie instances. When a user is followed, the establishing
of this relationship, unless the user is following someone on their
own instance, is sent to the other user's instance. The payload of 
this API path must have the shape illustrated below. An API token 
must exist first to follow someone.

```Typescript
{
    token: string,
    receiver_handle: string,
    receiver_host: string
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

## `POST /api/user/unfollow`

This API path allows users to unfollow users on their own instance or
other Valkyrie instances. When a user is unfollowed, the deletion
of this relationship, unless the user is unfollowing someone on their
own instance, is sent to the other user's instance. The payload of 
this API path must have the shape illustrated below. An API token 
must exist first to unfollow someone.

```Typescript
{
    token: string,
    receiver_handle: string,
    receiver_host: string
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
