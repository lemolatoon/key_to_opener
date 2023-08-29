use notify_rust::Notification;

pub fn default() -> notify_rust::Notification {
    let mut notification = Notification::new();
    notification
        .icon("key_to_opener")
        .timeout(3000)
        .appname("key to opener");

    notification
}
