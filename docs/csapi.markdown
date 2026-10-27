# VALKYRIE'S CLIENT-TO-SERVER API

The following section explains which client-to-server API paths this
implementation of the Valkyrie server has, which paths these API routes
exist, the payloads they expect, and the responses they send back. All
requests return responses in JSON form and those that are `POST`
requests expect JSON payloads.

## `GET /api/instance`

The response of this API path has the shape as illustrated below:

```JSON
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

```JSON
{
    "token": string
    "new_value": string
}
```

The response of this API path has the shape as illustrated below:


```JSON
{
    "is_ok": bool
}
```

## `POST /api/instance/edit/description`

The payload of this API path must have the shape illustrated below. The
token must be obtained first by calling the API route to login the
instance's administrator.


```JSON
{
    "token": string
    "new_value": string
}
```

The response of this API path has the shape as illustrated below:


```JSON
{
    "is_ok": bool
}
```

## `POST /api/user/create`

The payload of this API path must have the shape illustrated below:

```JSON
{
    "username": string,
    "password": string,
    "display_name": string,
    "description": string
}
```

The response of this API path has the shape as illustrated below:


```JSON
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

```JSON
{
    "username": string,
    "password": string
}
```

The response of this API path has the shape as illustrated below:


```JSON
{
    "token": string
}
```

## `POST /api/user/logout`

The payload of this API path must have the shape illustrated below:

```JSON
{
    "token": string
}
```

The response of this API path has the shape as illustrated below:


```JSON
{
    "is_ok": bool
}
```

## `POST /api/user/edit/name`

The payload of this API path must have the shape illustrated below. An
API token must be obtained first to carry out changes for a user.

```JSON
{
    "token": string
    "new_value": string
}
```

The response of this API path has the shape as illustrated below:


```JSON
{
    "is_ok": bool
}
```

## `POST /api/user/edit/description`

The payload of this API path must have the shape illustrated below. An
API token must be obtained first to carry out changes for a user.

```JSON
{
    "token": string
    "new_value": string
}
```

The response of this API path has the shape as illustrated below:


```JSON
{
    "is_ok": bool
}
```

## `POST /api/user/edit/password`

The payload of this API path must have the shape illustrated below:

```JSON
{
    "token": string,
    "password": string,
    "new_value": string
}
```

The response of this API path has the shape as illustrated below:


```JSON
{
    "is_ok": bool
}
```

## `POST /api/post/create`

The payload of this API path must have the shape illustrated below:

```JSON
{
    "token": string,
    "content": string
}
```

The response of this API path has the shape as illustrated below:


```JSON
{
    "post_id": string,
    "created_at": string, 
    "created_by": string, 
    "post_content": string
}
```
