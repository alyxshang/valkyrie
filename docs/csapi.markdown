# VALKYRIE'S CLIENT-TO-SERVER API

The following section explains which client-to-server API paths this
implementation of the Valkyrie server has, which paths these API routes
exist, the payloads they expect, and the responses they send back. All
requests return responses in Typescript form and those that are `POST`
requests expect Typescript payloads.

## `GET /api/instance`

The response of this API path has the shape as illustrated below:

```Typescript
{
    "name": string
    "admin": string
    "description": string
}
```

## `POST /api/instance/edit/name`

The payload of this API path must have the shape illustrated below. The
token must be obtained first by calling the API route to login the
instance's administrator.

```Typescript
{
    "token": string
    "new_value": string
}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "is_ok": bool
}
```

## `POST /api/instance/edit/description`

The payload of this API path must have the shape illustrated below. The
token must be obtained first by calling the API route to login the
instance's administrator.


```Typescript
{
    "token": string
    "new_value": string
}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "is_ok": bool
}
```

## `POST /api/user/create`

The payload of this API path must have the shape illustrated below:

```Typescript
{
    "username": string,
    "password": string,
    "display_name": string,
    "description": string
}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "username": string,
    "display_name": string,
    "description": string
}
```

## `POST /api/user/login`

The payload of this API path must have the shape illustrated below. This
API path can also be used to obtain an API token to carry out further
operations for a user.

```Typescript
{
    "username": string,
    "password": string
}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "token": string
}
```

## `POST /api/user/logout`

The payload of this API path must have the shape illustrated below:

```Typescript
{
    "token": string
}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "is_ok": bool
}
```

## `POST /api/user/edit/name`

The payload of this API path must have the shape illustrated below. An
API token must be obtained first to carry out changes for a user.

```Typescript
{
    "token": string
    "new_value": string
}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "is_ok": bool
}
```

## `POST /api/user/edit/description`

The payload of this API path must have the shape illustrated below. An
API token must be obtained first to carry out changes for a user.

```Typescript
{
    "token": string
    "new_value": string
}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "is_ok": bool
}
```

## `POST /api/user/edit/password`

The payload of this API path must have the shape illustrated below:

```Typescript
{
    "token": string,
    "password": string,
    "new_value": string
}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "is_ok": bool
}
```

## `POST /api/post/create`

The payload of this API path must have the shape illustrated below:

```Typescript
{
    "token": string,
    "content": string
}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "post_id": string,
    "created_at": string, 
    "created_by": string, 
    "post_content": string
}
```

## `POST /api/post/delete`

The payload of this API path must have the shape illustrated below:

```Typescript
{
    "token": string,
    "post_id": string
}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "is_ok": bool
}
```

## `POST /api/post/update`

The payload of this API path must have the shape illustrated below:

```Typescript
{
    "token": string,
    "content": string,
    "post_id": string
}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "is_ok": bool
}
```

## `POST /api/user/tokens/list`

The payload of this API path must have the shape illustrated below:

```Typescript
{
    "token": string,
}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "tokens": List<string>
}
```

## `POST /api/user/follow`

The payload of this API path must have the shape illustrated below:

```Typescript
{
    "token": string,
    "receiver_handle": string,
    "receiver_host": string

}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "is_ok": bool
}
```

## `POST /api/user/unfollow`

The payload of this API path must have the shape illustrated below:

```Typescript
{
    "token": string,
    "receiver_handle": string,
    "receiver_host": string

}
```

The response of this API path has the shape as illustrated below:


```Typescript
{
    "is_ok": bool
}
```
