# inetdx

Rust implementation of inetd internal services

| Service  | Port | RFC     | Notes                                |
| -------- | ---- | ------- | ------------------------------------ |
| Echo     | 7    | RFC 862 | -                                    |
| Discard  | 9    | RRC 863 | -                                    |
| Daytime  | 13   | RFC 867 | RFC 2822 timestamp                   |
| QotD     | 17   | RFC 865 | Custom list or /etc/motd as fallback |
| Chargen  | 19   | RFC 864 | -                                    |
| Time     | 37   | RFC 868 | Unix Timestamp as u64                |
| Hostname | 42   | -       | cat /etc/hostname over TCP+UDP       |