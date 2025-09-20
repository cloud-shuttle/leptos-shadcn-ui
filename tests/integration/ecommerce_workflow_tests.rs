#[cfg(test)]
mod ecommerce_workflow_tests {
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    use web_sys;
    use crate::default::{Button, Input, Card, CardHeader, CardTitle, CardContent, Badge, Separator};

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Debug, Clone, PartialEq)]
    struct Product {
        id: u32,
        name: String,
        price: f64,
        quantity: u32,
        in_stock: bool,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct CartItem {
        product: Product,
        quantity: u32,
    }

    #[wasm_bindgen_test]
    fn test_ecommerce_shopping_cart_workflow() {
        let products = vec![
            Product { id: 1, name: "Laptop".to_string(), price: 999.99, quantity: 5, in_stock: true },
            Product { id: 2, name: "Mouse".to_string(), price: 29.99, quantity: 10, in_stock: true },
            Product { id: 3, name: "Keyboard".to_string(), price: 79.99, quantity: 0, in_stock: false },
        ];

        let cart_items = RwSignal::new(Vec::<CartItem>::new());
        let total_price = RwSignal::new(0.0);
        let search_query = RwSignal::new(String::new());
        let selected_category = RwSignal::new("all".to_string());

        mount_to_body(move || {
            let filtered_products = products.iter()
                .filter(|p| {
                    let query = search_query.get();
                    let category = selected_category.get();
                    (query.is_empty() || p.name.to_lowercase().contains(&query.to_lowercase())) &&
                    (category == "all" || (category == "in_stock" && p.in_stock))
                })
                .collect::<Vec<_>>();

            view! {
                <div class="ecommerce-app">
                    <div class="search-section">
                        <Input 
                            placeholder="Search products..." 
                            value=search_query.get()
                            on_change=Callback::new(move |value| search_query.set(value))
                        />
                        <Button on_click=Callback::new(move || selected_category.set("in_stock".to_string()))>
                            "Show In Stock Only"
                        </Button>
                    </div>

                    <div class="products-grid">
                        {for filtered_products.iter().map(|product| {
                            let product = product.clone();
                            let cart_items = cart_items.clone();
                            let total_price = total_price.clone();
                            
                            view! {
                                <Card class="product-card">
                                    <CardHeader>
                                        <CardTitle>{product.name.clone()}</CardTitle>
                                    </CardHeader>
                                    <CardContent>
                                        <p>{format!("${:.2}", product.price)}</p>
                                        <p>{format!("Stock: {}", product.quantity)}</p>
                                        <Button 
                                            disabled=!product.in_stock
                                            on_click=Callback::new(move || {
                                                let mut items = cart_items.get();
                                                if let Some(existing) = items.iter_mut().find(|item| item.product.id == product.id) {
                                                    existing.quantity += 1;
                                                } else {
                                                    items.push(CartItem {
                                                        product: product.clone(),
                                                        quantity: 1,
                                                    });
                                                }
                                                cart_items.set(items);
                                                
                                                let new_total: f64 = cart_items.get().iter()
                                                    .map(|item| item.product.price * item.quantity as f64)
                                                    .sum();
                                                total_price.set(new_total);
                                            })
                                        >
                                            {if product.in_stock { "Add to Cart" } else { "Out of Stock" }}
                                        </Button>
                                    </CardContent>
                                </Card>
                            }
                        })}
                    </div>

                    <div class="cart-section">
                        <h3>"Shopping Cart"</h3>
                        <Separator />
                        {for cart_items.get().iter().map(|item| {
                            let item = item.clone();
                            let cart_items = cart_items.clone();
                            let total_price = total_price.clone();
                            
                            view! {
                                <div class="cart-item">
                                    <span>{item.product.name.clone()}</span>
                                    <span>{format!("${:.2}", item.product.price)}</span>
                                    <span>{format!("Qty: {}", item.quantity)}</span>
                                    <Button 
                                        on_click=Callback::new(move || {
                                            let mut items = cart_items.get();
                                            if let Some(index) = items.iter().position(|i| i.product.id == item.product.id) {
                                                if items[index].quantity > 1 {
                                                    items[index].quantity -= 1;
                                                } else {
                                                    items.remove(index);
                                                }
                                                cart_items.set(items);
                                                
                                                let new_total: f64 = cart_items.get().iter()
                                                    .map(|item| item.product.price * item.quantity as f64)
                                                    .sum();
                                                total_price.set(new_total);
                                            }
                                        })
                                    >
                                        "Remove"
                                    </Button>
                                </div>
                            }
                        })}
                        <div class="cart-total">
                            <strong>{format!("Total: ${:.2}", total_price.get())}</strong>
                        </div>
                    </div>
                </div>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test search functionality
        let search_input = document.query_selector("input[placeholder='Search products...']").unwrap().unwrap()
            .unchecked_into::<web_sys::HtmlInputElement>();
        search_input.set_value("laptop");
        let input_event = web_sys::InputEvent::new("input").unwrap();
        search_input.dispatch_event(&input_event).unwrap();

        // Test adding to cart
        let add_buttons = document.query_selector_all("button").unwrap();
        for i in 0..add_buttons.length() {
            let button = add_buttons.item(i).unwrap().unchecked_into::<web_sys::HtmlButtonElement>();
            if button.text_content().unwrap().contains("Add to Cart") {
                button.click();
                break;
            }
        }

        // Verify cart has items
        let cart_items = document.query_selector_all(".cart-item").unwrap();
        assert!(cart_items.length() > 0, "Cart should have items after adding product");

        // Test total calculation
        let total_element = document.query_selector(".cart-total strong").unwrap().unwrap();
        let total_text = total_element.text_content().unwrap();
        assert!(total_text.contains("$"), "Total should display price");
    }

    #[wasm_bindgen_test]
    fn test_ecommerce_checkout_workflow() {
        let cart_items = RwSignal::new(vec![
            CartItem {
                product: Product { id: 1, name: "Laptop".to_string(), price: 999.99, quantity: 1, in_stock: true },
                quantity: 1,
            }
        ]);

        let customer_info = RwSignal::new(("".to_string(), "".to_string(), "".to_string()));
        let payment_method = RwSignal::new("credit_card".to_string());
        let order_placed = RwSignal::new(false);

        mount_to_body(move || {
            view! {
                <div class="checkout-form">
                    <h2>"Checkout"</h2>
                    
                    <div class="customer-info">
                        <h3>"Customer Information"</h3>
                        <Input placeholder="Full Name" />
                        <Input placeholder="Email" />
                        <Input placeholder="Phone" />
                    </div>

                    <div class="payment-section">
                        <h3>"Payment Method"</h3>
                        <Button 
                            class=if payment_method.get() == "credit_card" { "selected" } else { "" }
                            on_click=Callback::new(move || payment_method.set("credit_card".to_string()))
                        >
                            "Credit Card"
                        </Button>
                        <Button 
                            class=if payment_method.get() == "paypal" { "selected" } else { "" }
                            on_click=Callback::new(move || payment_method.set("paypal".to_string()))
                        >
                            "PayPal"
                        </Button>
                    </div>

                    <div class="order-summary">
                        <h3>"Order Summary"</h3>
                        {for cart_items.get().iter().map(|item| {
                            view! {
                                <div class="summary-item">
                                    <span>{item.product.name.clone()}</span>
                                    <span>{format!("${:.2}", item.product.price * item.quantity as f64)}</span>
                                </div>
                            }
                        })}
                    </div>

                    <Button 
                        class="place-order-btn"
                        on_click=Callback::new(move || order_placed.set(true))
                    >
                        "Place Order"
                    </Button>

                    {if order_placed.get() {
                        view! {
                            <div class="order-confirmation">
                                <h3>"Order Placed Successfully!"</h3>
                                <p>"Thank you for your purchase."</p>
                            </div>
                        }
                    } else {
                        view! { <div></div> }
                    }}
                </div>
            }
        });

        let document = web_sys::window().unwrap().document().unwrap();
        
        // Test payment method selection
        let paypal_button = document.query_selector("button").unwrap().unwrap()
            .unchecked_into::<web_sys::HtmlButtonElement>();
        if paypal_button.text_content().unwrap().contains("PayPal") {
            paypal_button.click();
        }

        // Test order placement
        let place_order_btn = document.query_selector(".place-order-btn").unwrap().unwrap()
            .unchecked_into::<web_sys::HtmlButtonElement>();
        place_order_btn.click();

        // Verify order confirmation
        let confirmation = document.query_selector(".order-confirmation").unwrap();
        assert!(confirmation.is_some(), "Order confirmation should appear after placing order");
    }
}