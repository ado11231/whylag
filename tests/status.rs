use whylag::status::{Health, evaluate};

#[test]
fn status_follows_connection_checks_in_order() {
    let cases = [
        (
            false,
            None,
            None,
            Some(false),
            None,
            0,
            "Not connected: no router found",
        ),
        (
            true,
            None,
            None,
            Some(false),
            None,
            0,
            "Wifi problem: can't reach router",
        ),
        (
            true,
            Some(4.0),
            None,
            Some(false),
            None,
            0,
            "Internet unreachable: check your provider",
        ),
        (
            true,
            Some(4.0),
            Some(20.0),
            Some(false),
            None,
            0,
            "DNS problem: websites may not load",
        ),
        (
            true,
            Some(4.0),
            Some(20.0),
            Some(true),
            Some(-71),
            0,
            "Weak wifi",
        ),
        (
            true,
            Some(51.0),
            Some(20.0),
            Some(true),
            None,
            0,
            "Weak wifi",
        ),
        (
            true,
            Some(4.0),
            Some(151.0),
            Some(true),
            None,
            0,
            "Internet laggy",
        ),
        (
            true,
            Some(4.0),
            Some(20.0),
            Some(true),
            None,
            1,
            "Internet laggy",
        ),
    ];
    for (found, router, internet, dns, signal, loss, expected) in cases {
        assert_eq!(
            evaluate(found, Some(router), Some(internet), dns, signal, loss).message,
            expected
        );
    }
}

#[test]
fn values_at_the_threshold_remain_healthy() {
    assert_eq!(
        evaluate(
            true,
            Some(Some(50.0)),
            Some(Some(150.0)),
            Some(true),
            Some(-70),
            0
        )
        .health,
        Health::Good
    );
}
