# Многопоточность. Go

## Описание

 `Одновременное извлечение данных из нескольких URL-адресов`

* считывание url адресов из текстового файла
* запуск отдельной подпрограммы для каждого url, чтобы одновременно извлекать данные
* `fetchData`: основная функция, которая отвечает за отправку HTTP GET запроса и обработку ответа. Проверяет наличие ошибок на этапах запроса и ответа, регистрируя ошибки и фиксируя результаты
* `sync.WaitGoup` используется для гарантии того, что программа ожидает завершения всех программ

## Запуск

```bash
docker build -t go_threads_image .
docker run -p 8080:8080 go_threads_image
```
