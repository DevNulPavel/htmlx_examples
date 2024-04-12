use crate::{data::user::User, error::CommonError};
use std::{
    fs::File,
    io::{BufReader, BufWriter, ErrorKind},
    path::Path,
};

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn load_users(users_file_path: &Path) -> Result<Vec<User>, CommonError> {
    // Для этого примера достаточно простого синхронного чтения файлика, вообще без изысков.
    let reader = {
        // Откроем файлик, проверим его существование
        let file = match File::open(users_file_path) {
            // Файлик открыли
            Ok(file) => file,
            // Файлика нету
            Err(err) if err.kind() == ErrorKind::NotFound => {
                return Ok(Vec::new());
            }
            // Ошибки прочие
            Err(err) => {
                return Err(err.into());
            }
        };

        // Создаем буфферизацию чтения
        BufReader::new(file)
    };

    // Парсим данные
    let users = serde_json::from_reader::<_, Vec<User>>(reader)?;

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
    serde_json::to_writer(&mut file_writer, &users)?;

    // Возвращаем назад файлик для атомарной замены
    // Здесь не будем сохранять в ошибке непосредтвенно сам writer, оставим лишь IO ошибку
    let temp_file = file_writer
        .into_inner()
        .map_err(|err| CommonError::IntoInner(err.into_error()))?;

    // Атомарно заменяем временный файлик на постоянный
    temp_file.persist(users_file_path)?;

    Ok(())
}
