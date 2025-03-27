use klyra_service::{Error, Service};

#[derive(Clone)]
pub struct MyService;

#[klyra_service::async_trait]
impl Service for MyService {
    async fn bind(
        mut self: Box<Self>,
        _addr: std::net::SocketAddr,
    ) -> Result<(), klyra_service::error::Error> {
        println!("service is binding");
        Ok(())
    }
}

#[klyra_service::main]
async fn klyra() -> Result<MyService, Error> {
    Ok(MyService {})
}

// async fn __klyra_wrapper(
//     _factory: &mut dyn klyra_service::Factory,
//     runtime: &klyra_service::Runtime,
//     logger: Box<dyn klyra_service::log::Log>,
// ) -> Result<Box<dyn Service>, Error> {
//     runtime
//         .spawn_blocking(move || {
//             klyra_service::log::set_boxed_logger(logger)
//                 .map(|()| {
//                     klyra_service::log::set_max_level(klyra_service::log::LevelFilter::Info)
//                 })
//                 .expect("logger set should succeed");
//         })
//         .await
//         .map_err(|e| {
//             if e.is_panic() {
//                 let mes = e
//                     .into_panic()
//                     .downcast_ref::<&str>()
//                     .map(|x| x.to_string())
//                     .unwrap_or_else(|| "<no panic message>".to_string());

//                 klyra_service::Error::BuildPanic(mes)
//             } else {
//                 klyra_service::Error::Custom(klyra_service::error::CustomError::new(e))
//             }
//         })?;

//     runtime
//         .spawn(async {
//             klyra()
//                 .await
//                 .map(|ok| Box::new(ok) as Box<dyn klyra_service::Service>)
//         })
//         .await
//         .map_err(|e| {
//             if e.is_panic() {
//                 let mes = e
//                     .into_panic()
//                     .downcast_ref::<&str>()
//                     .map(|x| x.to_string())
//                     .unwrap_or_else(|| "<no panic message>".to_string());

//                 klyra_service::Error::BuildPanic(mes)
//             } else {
//                 klyra_service::Error::Custom(klyra_service::error::CustomError::new(e))
//             }
//         })?
// }

// fn __binder(
//     service: Box<dyn klyra_service::Service>,
//     addr: std::net::SocketAddr,
//     runtime: &klyra_service::Runtime,
// ) -> klyra_service::ServeHandle {
//     runtime.spawn(async move { service.bind(addr).await })
// }

// #[no_mangle]
// pub extern "C" fn _create_service() -> *mut klyra_service::Bootstrapper {
//     let builder: klyra_service::StateBuilder<Box<dyn klyra_service::Service>> =
//         |factory, runtime, logger| Box::pin(__klyra_wrapper(factory, runtime, logger));

//     let bootstrapper = klyra_service::Bootstrapper::new(
//         builder,
//         __binder,
//         klyra_service::Runtime::new().unwrap(),
//     );

//     let boxed = Box::new(bootstrapper);
//     Box::into_raw(boxed)
// }
