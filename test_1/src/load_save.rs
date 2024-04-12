use crate::{data::user::User, error::CommonError};
use std::{
    fs::File,
    io::{BufWriter, Read},
    path::Path,
};

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn load_users(users_file_path: &Path) -> Result<Vec<User>, CommonError> {
    // Для этого примера достаточно простого синхронного чтения файлика, вообще без изысков.
    let file_data = {
        // Откроем файлик, проверим его существование
        let mut file = match File::open(users_file_path) {
            // Файлик открыли
            Ok(file) => file,
            // TODO: Сейчас лучше будем кидать ошибку
            // Файлика нету
            // Err(err) if err.kind() == ErrorKind::NotFound => {
            //     return Ok(Vec::new());
            // }
            // Ошибки прочие
            Err(err) => {
                return Err(err.into());
            }
        };

        // Размер файлика
        let data_len: usize = {
            let file_len: u64 = file.metadata()?.len();

            // TODO: usize же может быть 32 бита, ошибка же должна быть?
            cast::usize(file_len)
        };

        // Буфер
        let mut buf = Vec::with_capacity(data_len);

        // Читаем данные все в оперативку
        file.read_to_end(&mut buf)?;

        buf
    };

    // Парсим данные из оперативки, это быстрее, чем из reader
    let users = serde_json::from_slice::<Vec<User>>(&file_data)?;

    Ok(users)
}

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn save_users(users: Vec<User>, users_file_path: &Path) -> Result<(), CommonError> {
    // Создадим директорию для файлика если ее не существует еще.
    // затем содзадим временный файлик.
    // Для относительных путей в текущей директории возвращается Some("")
    let temp_file = if let Some(folder) = users_file_path.parent() {
        // Создаем директории
        if !folder.exists() {
            std::fs::create_dir_all(folder)?;
        }

        // Создаем временный файлик в директории
        tempfile::NamedTempFile::new_in(folder)?
    } else {
        // Создаем временный файл в системе если почему-то у нас None в виде корня
        tempfile::NamedTempFile::new()?
    };

    // Создаем обертку для буферизации
    let mut file_writer = BufWriter::new(temp_file);

    // Запишем сохранение
    serde_json::to_writer_pretty(&mut file_writer, &users)?;

    // Возвращаем назад файлик для атомарной замены
    // Здесь не будем сохранять в ошибке непосредтвенно сам writer, оставим лишь IO ошибку
    let temp_file = file_writer
        .into_inner()
        .map_err(|err| CommonError::IntoInner(err.into_error()))?;

    // Атомарно заменяем временный файлик на постоянный
    temp_file.persist(users_file_path)?;

    Ok(())
}
