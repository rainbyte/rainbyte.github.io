---
title: The HTTP Language
author: rainbyte
published: 2022-08-03 14:30:00
tags: api, http, request, vscode
language: en
---

It is typical to see the situation where a developer has to work with services which provide an HTTP api.

Suppose we are implementing the following common endpoints:

- `GET /todos`: list all the Todos
- `POST /todos`: create a new Todo
- `PUT /todos/:id`: update a Todo indicated by :id
- `DELETE /todos/:id`: remove a Todo indicated by :id

After using a particular programming language to achieve the task, it is
possible that we would like to communicate with it to test it works fine.

There are many tools in the market which allows us to make requests, even
in a graphical point and click way, but I have found more confortable to
have a text representation.

There is a file format called `HTTP language` which can represent requests
in an standardized fashion and it is supported by CLI tools like `httpYac`
and editor extensions like `vscode-restclient`.

The `httpYac` tool can be installed using `npm` with the following command:

```sh
npm -g install httpyac
```

I wrote a few examples about how to make requests to the proposed HTTP api
using the `HTTP language` format. It can be seen in the following snippet
that the syntax is very simple and complies with IETF request line spec.

## HTTP language examples

We can obtain a single Todo using the `GET` verb followed by the host and
the port where server is running, the HTTP version is optional:

```http
GET http://localhost:4000/todos HTTP/1.1
```

To create a new Todo task we can follow the example and use `POST` verb, and
in this case the `Content-Type` header should be specified, given that we
are attaching a JSON data structure as body of the request:

```http
POST http://localhost:4000/todos HTTP/1.1
Content-Type: application/json

{
    "text": "blabla",
    "completed": false
}
```

If we want to obtain a single Todo task then a variable could be used to
indicate the `:id`, and the value can be accessed using brackets:

```http
@get-todo-id = a0804e5f-a849-4920-9023-557ecdd790d1
GET http://localhost:4000/todos/{{get-todo-id}} HTTP/1.1
```

To modify a Todo task we use the `PUT` verb as appears on the proposed API.
It is important to have the `Content-Type` header defined as we use JSON:

```http
@put-todo-id = a0804e5f-a849-4920-9023-557ecdd790d1
PUT http://localhost:4000/todos/{{put-todo-id}} HTTP/1.1
Content-Type: application/json

{
    "text": "foobar",
    "completed": false
}
```

The format can also be used with the `DELETE` verb, and in this case we use
a variable one more time, but with different name to avoid conflicts:

```http
@del-todo-id = a0804e5f-a849-4920-9023-557ecdd790d1
DELETE http://localhost:4000/todos/{{del-todo-id}} HTTP/1.1
```

These snippets can be copied to a file with
`.rest` or `.http` extension, eg. `todo-requests.http`, and executed this way:

```sh
httpyac todo-requests.http
```

Happy hacking! 🐱