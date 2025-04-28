# Kubernetes Operator POC

Trying to learn how to make operators in kubernetes using `kube.rs` and how to
manage the whole environment using nix.

## What's supported

* Building rust based projects
* Building OCI images of the rust projects
* Setting up a local kubernetes cluster

## Specs

A simple REST API that returns `Krangles`. Operator will fetch these using an
API and maintain their state as kubernetes objects.

All of these properties should be visible in Kubernetes objects and it should be
able to corrupt and uncorrupt (heresy) a `Krangle` through Kubernetes. On top of
these Kubernetes objects maintain a `connected` boolean stating if the operator
was able to create a connection with the said `Krangle`.

When checking for connection `Krangles` with id 1 and 42 should always fail to
connect (timeout). `Krangle` id 3 should flicker randomly (connection refused).

## Krangles REST API Documentation

This API provides access to **Krangles**, allowing clients to retrieve and
modify their state.

### Base URL

```
http://localhost:6666/api
```

---

### Endpoints

#### 1. Get All Krangles

**GET** `/krangles`

Retrieve a list of all Krangles.

##### Response

- **Status Code**: `200 OK`
- **Content-Type**: `application/json`

```json
[
  {
    "id": 1,
    "address": "http://example.com/krangle/1",
    "corrupted": false,
    "enabled": true
  },
  {
    "id": 2,
    "address": "http://example.com/krangle/2",
    "corrupted": true,
    "enabled": false
  }
]
```

---

#### 2. Corrupt a Krangle

**POST** `/krangles/{id}/corrupt`

Marks a specific Krangle as corrupted.

##### Parameters

- **Path Parameter**:
  - `id` (integer): The unique identifier of the Krangle to corrupt.

##### Request

- **Content-Type**: `application/json`
- **Body**: Empty

##### Response

- **Status Code**: `200 OK`
- **Content-Type**: `application/json`

```json
{
  "id": 1,
  "address": "http://example.com/krangle/1",
  "corrupted": true,
  "enabled": true
}
```

##### Errors

- `404 Not Found` — Krangle with specified ID does not exist.
- `400 Bad Request` — Invalid input or already corrupted.

---

### Krangle Object

| Field       | Type    | Description                      |
|------------|---------|----------------------------------|
| `id`       | integer | Unique identifier                |
| `address`  | string  | URL associated with Krangle      |
| `corrupted`| boolean | Whether the Krangle is corrupted |
| `enabled`  | boolean | Whether the Krangle is enabled   |
