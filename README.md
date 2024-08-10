# Devil

**Devil** is a robust and efficient asynchronous wrapper for the `libudev`
library provided by `systemd`. Designed to integrate seamlessly with the `Tokio`
ecosystem, Devil simplifies the process of monitoring device changes on Linux
systems.

## Purpose

The primary goal of Devil is to offer a straightforward and reliable solution
for tracking device events and changes in the system. By leveraging asynchronous
programming, Devil ensures that your applications remain responsive while
handling device monitoring tasks.

## Key Users

Devil is already mainly utilized in the following projects:

- [**Colpetto**](https://github.com/verdiwm/colpetto): An asynchronous wrapper
  for `libinput` that internally relies on `libudev` through Devil for device
  event handling.

- [**Diretto**](https://github.com/verdiwm/diretto): A wrapper for the Linux
  `DRM` (Direct Rendering Manager) interface, where Devil is employed to monitor
  devices capable of `DRM`.

## Future Plans

Looking ahead, Devil aims to expand its capabilities by providing wrappers for
the new `sd-device` interface introduced by `systemd`. While development is in
progress, there is no fixed ETA for these updates.

## License

This project is licensed under the
[Apache-2.0 License](http://www.apache.org/licenses/LICENSE-2.0). For more
information, please see the [LICENSE](LICENSE) file.
