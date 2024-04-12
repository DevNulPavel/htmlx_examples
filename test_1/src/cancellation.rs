use tokio_util::sync::CancellationToken;

//////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct ShutdownTokens {
    pub cancelation_grace: CancellationToken,
    pub cancelation_force: CancellationToken,
}

//////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn spawn_gracefull_shutdown() -> ShutdownTokens {
    // Создаем токены
    let cancelation_grace = CancellationToken::new();
    let cancelation_force = CancellationToken::new();

    {
        // Просто клонируем токен, не создавая дочерний
        let cancelation_grace = cancelation_grace.clone();
        let cancelation_force = cancelation_force.clone();

        // Запускаем в работу фоновую отмену работы
        tokio::spawn(async move {
            loop {
                tokio::signal::ctrl_c().await.unwrap();

                // Ставим пометку о завершении, новые задачи уже не будем брать
                println!("SIGINT signal received, wait for started tasks");
                cancelation_grace.cancel();

                tokio::signal::ctrl_c().await.unwrap();

                // Принудительное завершение
                println!("\nFORCE EXIT\n");
                cancelation_force.cancel();
            }
        });
    }

    ShutdownTokens {
        cancelation_grace,
        cancelation_force,
    }
}
