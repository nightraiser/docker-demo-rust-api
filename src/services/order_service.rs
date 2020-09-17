use crate::dtos::order::Order;

pub fn get_all_orders() -> Vec<Order> {
    vec![
        Order {
            id:1,
            title: "my order".to_string()
        },
        Order {
            id:2,
            title: "my order".to_string()
        }
    ]
}

// pub fn get_order(id: i32) -> Option<Order> {
//     let orders = [
//         Order {
//             id:1,
//             title: "my order".to_string()
//         },
//         Order {
//             id:2,
//             title: "my order".to_string()
//         }
//     ];
//     let order = orders.iter().find(|&&x| x.id == id);
//     return order;
// }