pub fn animation_style(duration: u32) -> String {
    format! {"
        .bet-animation-a {{
            animation: betNotificationA {duration}s ease-in-out forwards;
        }}
        
        .bet-animation-b {{
             animation: betNotificationB {duration}s ease-in-out forwards;
        }}
    " 
    }
}