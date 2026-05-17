// ============================================================
// CONCEPT: State Machine with Enums
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# state machines: switch statements, state pattern (interface +
// concrete classes), or source-generator-based approaches.
//
// Rust: Enums with data per variant are the natural fit.
// The compiler enforces exhaustive matching, so adding a new
// state causes compile errors at every match site — no forgotten transitions.
//
// RUN: cargo run --bin state_machine
// ============================================================

fn main() {
    println!("=== State Machine with Enums ===\n");

    order_state_machine();
    traffic_light();
    tcp_connection();
}

// ---- 1. Order processing state machine ----------------------------

#[derive(Debug, Clone, PartialEq)]
enum OrderStatus {
    Pending { items: Vec<String> },
    Confirmed { order_id: u64, items: Vec<String> },
    Shipped { tracking: String, order_id: u64 },
    Delivered { rating: Option<u8> },
    Cancelled { reason: String },
}

impl OrderStatus {
    fn confirm(self, order_id: u64) -> Result<Self, String> {
        match self {
            OrderStatus::Pending { items } if !items.is_empty() =>
                Ok(OrderStatus::Confirmed { order_id, items }),
            OrderStatus::Pending { .. } =>
                Err("cannot confirm empty order".to_string()),
            other =>
                Err(format!("cannot confirm order in state {:?}", std::mem::discriminant(&other))),
        }
    }

    fn ship(self, tracking: String) -> Result<Self, String> {
        match self {
            OrderStatus::Confirmed { order_id, .. } =>
                Ok(OrderStatus::Shipped { tracking, order_id }),
            other =>
                Err(format!("cannot ship from {:?}", std::mem::discriminant(&other))),
        }
    }

    fn deliver(self) -> Result<Self, String> {
        match self {
            OrderStatus::Shipped { .. } =>
                Ok(OrderStatus::Delivered { rating: None }),
            other =>
                Err(format!("cannot deliver from {:?}", std::mem::discriminant(&other))),
        }
    }

    fn cancel(self, reason: impl Into<String>) -> Result<Self, String> {
        match self {
            OrderStatus::Pending { .. } | OrderStatus::Confirmed { .. } =>
                Ok(OrderStatus::Cancelled { reason: reason.into() }),
            OrderStatus::Shipped { .. } =>
                Err("cannot cancel shipped order".to_string()),
            other =>
                Err(format!("cannot cancel from {:?}", std::mem::discriminant(&other))),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Pending   { .. } => "Pending",
            Self::Confirmed { .. } => "Confirmed",
            Self::Shipped   { .. } => "Shipped",
            Self::Delivered { .. } => "Delivered",
            Self::Cancelled { .. } => "Cancelled",
        }
    }
}

fn order_state_machine() {
    println!("--- Order Processing State Machine ---");

    let order = OrderStatus::Pending {
        items: vec!["Rust book".to_string(), "Keyboard".to_string()],
    };
    println!("Initial: {}", order.name());

    let order = order.confirm(1001).unwrap();
    println!("After confirm: {}", order.name());

    let order = order.ship("TRACK-XYZ-123".to_string()).unwrap();
    println!("After ship: {}", order.name());

    let order = order.deliver().unwrap();
    println!("After deliver: {}", order.name());

    // Invalid transition:
    let bad = OrderStatus::Pending { items: vec![] };
    match bad.ship("TRACK-1".to_string()) {
        Ok(_)  => println!("shipped"),
        Err(e) => println!("error (expected): {e}"),
    }
}

// ---- 2. Traffic light --------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
enum Light { Red, Yellow, Green }

impl Light {
    fn next(self) -> Light {
        match self {
            Light::Red    => Light::Green,
            Light::Green  => Light::Yellow,
            Light::Yellow => Light::Red,
        }
    }

    fn duration_secs(self) -> u32 {
        match self {
            Light::Red    => 60,
            Light::Green  => 45,
            Light::Yellow => 5,
        }
    }

    fn can_proceed(self) -> bool {
        matches!(self, Light::Green)
    }
}

impl std::fmt::Display for Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Light::Red    => "RED   ",
            Light::Green  => "GREEN ",
            Light::Yellow => "YELLOW",
        };
        write!(f, "{s}")
    }
}

fn traffic_light() {
    println!("\n--- Traffic Light State Machine ---");

    let mut light = Light::Red;
    for _ in 0..6 {
        println!("  {light} ({:2}s) proceed={}", light.duration_secs(), light.can_proceed());
        light = light.next();
    }
}

// ---- 3. TCP connection state machine --------------------------------

#[derive(Debug, Clone, PartialEq)]
enum TcpState {
    Closed,
    Listen,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    TimeWait,
}

#[derive(Debug)]
enum TcpEvent {
    PassiveOpen,   // server starts listening
    SynReceived,   // SYN packet received
    AckReceived,   // ACK received
    Send,          // application sends data
    Close,         // application calls close
    FinReceived,   // FIN from remote
}

#[derive(Debug, thiserror::Error)]
#[error("invalid TCP transition: {event:?} in state {state:?}")]
struct TcpError { state: String, event: String }

fn tcp_transition(state: TcpState, event: TcpEvent) -> Result<TcpState, TcpError> {
    let err = |s: &TcpState, e: &TcpEvent| TcpError {
        state: format!("{s:?}"),
        event: format!("{e:?}"),
    };

    match (&state, &event) {
        (TcpState::Closed,       TcpEvent::PassiveOpen)  => Ok(TcpState::Listen),
        (TcpState::Listen,       TcpEvent::SynReceived)  => Ok(TcpState::SynReceived),
        (TcpState::SynReceived,  TcpEvent::AckReceived)  => Ok(TcpState::Established),
        (TcpState::Established,  TcpEvent::Close)        => Ok(TcpState::FinWait1),
        (TcpState::FinWait1,     TcpEvent::AckReceived)  => Ok(TcpState::FinWait2),
        (TcpState::FinWait2,     TcpEvent::FinReceived)  => Ok(TcpState::TimeWait),
        _ => Err(err(&state, &event)),
    }
}

fn tcp_connection() {
    println!("\n--- Simplified TCP State Machine ---");

    let transitions = [
        TcpEvent::PassiveOpen,
        TcpEvent::SynReceived,
        TcpEvent::AckReceived,
        TcpEvent::Close,
        TcpEvent::AckReceived,
        TcpEvent::FinReceived,
    ];

    let mut state = TcpState::Closed;
    for event in transitions {
        let event_name = format!("{event:?}");
        match tcp_transition(state.clone(), event) {
            Ok(next) => {
                println!("  {event_name:15} -> {next:?}");
                state = next;
            }
            Err(e) => println!("  error: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_happy_path() {
        let o = OrderStatus::Pending { items: vec!["item".to_string()] };
        let o = o.confirm(1).unwrap();
        let o = o.ship("T1".to_string()).unwrap();
        let o = o.deliver().unwrap();
        assert_eq!(o, OrderStatus::Delivered { rating: None });
    }

    #[test]
    fn cannot_ship_pending() {
        let o = OrderStatus::Pending { items: vec![] };
        assert!(o.ship("T1".to_string()).is_err());
    }

    #[test]
    fn light_cycle() {
        let l = Light::Red.next().next().next();
        assert_eq!(l, Light::Red);
    }
}
