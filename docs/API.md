# API
<!-- markdownlint-disable MD024 -->

## `/version`

Gets the running version of turso-bin

```shell
curl http://127.0.0.1:8080/version
```

## `/paste/by_id/{id}`

Get a paste based on its ID

### Example

```shell
curl http://127.0.0.1:8080/paste/by_id/1
```

## `/paste/by_link/

```shell
curl http://127.0.0.1:8080/paste/by_id/BDEEA
```

## `/pastes/`

Lists pastes

### Example

```shell
curl http://127.0.0.1:8080/pastes/
```

## `/pastes/create`

Create a paste

### Example

```shell
curl http://127.0.0.1:8080/pastes/create \
--request GET \
--header 'Content-Type: application/json' \
--data '"foobar from curl"'
```
