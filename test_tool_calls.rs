// 简单测试脚本验证工具调用修复
use serde_json;

fn main() {
    // 模拟API响应中的工具调用数据
    let response_json = serde_json::json!({
        "choices": [{
            "message": {
                "content": "我来帮你执行工具调用",
                "tool_calls": [{
                    "id": "call_123",
                    "type": "function", 
                    "function": {
                        "name": "test_tool",
                        "arguments": "{\"param\": \"value\"}"
                    }
                }]
            }
        }]
    });

    // 测试解析content
    let content = response_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();
    
    println!("解析的content: {}", content);

    // 测试解析tool_calls
    let tool_calls = response_json["choices"][0]["message"]["tool_calls"]
        .as_array()
        .and_then(|calls| {
            let parsed_calls: Result<Vec<_>, _> = calls
                .iter()
                .map(|call| {
                    Ok(super::ToolCall {
                        id: call["id"].as_str().unwrap_or("").to_string(),
                        tool_type: call["type"].as_str().unwrap_or("function").to_string(),
                        function: super::FunctionCall {
                            name: call["function"]["name"].as_str().unwrap_or("").to_string(),
                            arguments: call["function"]["arguments"].as_str().unwrap_or("").to_string(),
                        },
                    })
                })
                .collect();

            parsed_calls.ok()
        });

    match tool_calls {
        Some(calls) => {
            println!("解析到 {} 个工具调用:", calls.len());
            for call in calls {
                println!("  - ID: {}, 类型: {}, 函数: {}", call.id, call.tool_type, call.function.name);
            }
        }
        None => println!("未找到工具调用"),
    }
}

// 定义与history模块中相同的结构体用于测试
#[derive(Debug)]
struct ToolCall {
    id: String,
    tool_type: String,
    function: FunctionCall,
}

#[derive(Debug)]
struct FunctionCall {
    name: String,
    arguments: String,
}