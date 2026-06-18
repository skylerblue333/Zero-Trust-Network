use std::collections::HashMap;

struct Identity {
    id: String,
    role: String,
    device_health_score: u8,
}

struct ZeroTrustPolicyEngine {
    resource_policies: HashMap<String, u8>,
}

impl ZeroTrustPolicyEngine {
    fn new() -> Self {
        let mut policies = HashMap::new();
        policies.insert("financial_records".to_string(), 90);
        policies.insert("public_docs".to_string(), 10);
        ZeroTrustPolicyEngine { resource_policies: policies }
    }

    fn evaluate_access(&self, identity: &Identity, resource: &str) -> bool {
        println!("Evaluating access for {} to {}", identity.id, resource);
        
        if let Some(&required_score) = self.resource_policies.get(resource) {
            if identity.device_health_score >= required_score {
                println!("Access GRANTED: Device health {} >= {}", identity.device_health_score, required_score);
                return true;
            } else {
                println!("Access DENIED: Device health {} < {}", identity.device_health_score, required_score);
                return false;
            }
        }
        println!("Access DENIED: Resource unknown");
        false
    }
}

fn main() {
    let engine = ZeroTrustPolicyEngine::new();
    
    let user1 = Identity {
        id: "user_123".to_string(),
        role: "employee".to_string(),
        device_health_score: 95, // Fully patched, secure device
    };
    
    let user2 = Identity {
        id: "user_456".to_string(),
        role: "contractor".to_string(),
        device_health_score: 50, // Missing patches
    };
    
    engine.evaluate_access(&user1, "financial_records");
    engine.evaluate_access(&user2, "financial_records");
}
