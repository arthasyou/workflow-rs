# HTTP Node 使用指南

## 概述

HTTP 节点允许在工作流中发送 HTTP 请求到远程 API 服务器，并处理响应数据。

## 配置参数

HTTP 节点支持以下配置参数：

```json
{
  "url": "https://api.example.com/endpoint",  // 必需：API 端点 URL
  "input_data": {                              // 必需：请求数据
    "Single": {
      "Json": {
        "key": "value"
      }
    }
  },
  "method": "POST",                            // 可选：HTTP 方法（默认 POST）
  "headers": {                                 // 可选：请求头
    "Content-Type": "application/json",
    "Authorization": "Bearer token"
  },
  "timeout_seconds": 30                        // 可选：超时时间（默认 30 秒）
}
```

## 支持的 HTTP 方法

- GET
- POST
- PUT
- DELETE
- PATCH

## 数据合并

如果节点接收到输入数据，并且输入数据和配置中的 `input_data` 都是 JSON 对象，它们会被合并。配置中的数据会覆盖输入数据中的同名字段。

## 响应处理

- 如果响应是有效的 JSON，将直接返回 JSON 数据
- 如果响应不是 JSON，将返回包含原始文本和状态码的对象：
  ```json
  {
    "raw": "响应文本",
    "status": 200
  }
  ```

## 示例配置

### 1. 简单的 GET 请求

```json
{
  "id": "get_user",
  "node_type": {
    "Data": "Http"
  },
  "data": {
    "url": "https://api.example.com/users/123",
    "input_data": {
      "Single": {
        "Json": {}
      }
    },
    "method": "GET",
    "headers": {
      "Accept": "application/json"
    }
  }
}
```

### 2. 带认证的 POST 请求

```json
{
  "id": "create_resource",
  "node_type": {
    "Data": "Http"
  },
  "data": {
    "url": "https://api.example.com/resources",
    "input_data": {
      "Single": {
        "Json": {
          "name": "New Resource",
          "type": "example"
        }
      }
    },
    "method": "POST",
    "headers": {
      "Content-Type": "application/json",
      "Authorization": "Bearer your-api-token"
    },
    "timeout_seconds": 60
  }
}
```

### 3. 在工作流中使用

```json
{
  "nodes": [
    {
      "id": "start",
      "node_type": { "Data": "Input" },
      "data": {
        "input": {
          "Single": {
            "Json": {
              "user_id": "123"
            }
          }
        }
      }
    },
    {
      "id": "fetch_user",
      "node_type": { "Data": "Http" },
      "data": {
        "url": "https://jsonplaceholder.typicode.com/users/1",
        "input_data": {
          "Single": {
            "Json": {}
          }
        },
        "method": "GET"
      }
    },
    {
      "id": "end",
      "node_type": { "Data": "Identity" },
      "data": null
    }
  ],
  "edges": [
    { "source": "start", "target": "fetch_user" },
    { "source": "fetch_user", "target": "end" }
  ]
}
```

## 错误处理

HTTP 节点会在以下情况返回错误：

1. 无法创建 HTTP 客户端
2. 请求失败（网络错误等）
3. 响应状态码不是 2xx
4. 无法读取响应内容

错误信息会包含具体的失败原因，便于调试。