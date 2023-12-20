# :books: bookhouse
<!--[![Static Badge](https://img.shields.io/badge/README-ru-green)]()-->
<!--[![Static Badge](https://img.shields.io/badge/README-en-red)]()-->



## <a id="overview">Обзор</a>
bookhouse - информационная система для мелкооптового книжного магазина.



## <a id="toc">Содержание</a>
* [Обзор](#overview)
* [Содержание](#toc)
* [Описание предметной области](#domain-description)
    + [Поставка книг на склад](#delivery-of-books-to-warehouse)
    + [Учет книг на складе](#-------------------)
    + [Оформление заказов](#------------------)



## <a id="domain-description">Описание предметной области</a>
В магазине продаются книги, которые хранятся на складах. Магазин закупает свой товар у поставщиков и регистрирует поступление новых партий книг. Продажи книг осуществляются на основе заказов, которые могут быть оформлены как физическими, так и юридическими лицами.

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://www.plantuml.com/plantuml/dpng/TPF1Qjmm48RlUegXbzxo37cJeBMdiOXi2KdkOkf0qaDBeOVUMdefFVLMJDXjWfVu6KPlrB45QtMtCTXuvlz-Z_NRklH1kj3MMeZNKAaI8LG8tfeWKbaeLsXaKRurgWJhp4PZxM6hGWMokTPOGk7CriCRn23yZMgiTB86hHeCdKKmBbH626o22Xsy0lf23_4zJV6E2-b1jgox302Wx_HCJ_I7hnFjWG-k7cb7mzBdJh9kZBcvmb85PPfymhS4yvGz7UBdUCzt3pGbasqtMlaAdSWjFsXQi871aOQ4-jPQtK66_MJnmD0k0Llfi3Ajnvpvolh8vsDYZ2lHmGkpuYTUpvXDK-WoydUyOtr7J_4ZxUCzZKiUTE36X-DwgiwhGkehYywR9dF_ec6Py_mVbyRhrIB4CHJe-ukB_js_qQPF1BSicHD9vDLCw1b9mPrfBAsHdQcEvPIM44TgVf_rYMl_r93qPKZVhr6cy6_Dt3rBAXTF0bd-_Hhh3TVY4fjotY9_0G00">
    <source media="(prefers-color-scheme: light)" srcset="https://www.plantuml.com/plantuml/png/TPF1Qjmm48RlUegXbzxo37cJeBMdiOXi2KdkOkf0qaDBeOVUMdefFVLMJDXjWfVu6KPlrB45QtMtCTXuvlz-Z_NRklH1kj3MMeZNKAaI8LG8tfeWKbaeLsXaKRurgWJhp4PZxM6hGWMokTPOGk7CriCRn23yZMgiTB86hHeCdKKmBbH626o22Xsy0lf23_4zJV6E2-b1jgox302Wx_HCJ_I7hnFjWG-k7cb7mzBdJh9kZBcvmb85PPfymhS4yvGz7UBdUCzt3pGbasqtMlaAdSWjFsXQi871aOQ4-jPQtK66_MJnmD0k0Llfi3Ajnvpvolh8vsDYZ2lHmGkpuYTUpvXDK-WoydUyOtr7J_4ZxUCzZKiUTE36X-DwgiwhGkehYywR9dF_ec6Py_mVbyRhrIB4CHJe-ukB_js_qQPF1BSicHD9vDLCw1b9mPrfBAsHdQcEvPIM44TgVf_rYMl_r93qPKZVhr6cy6_Dt3rBAXTF0bd-_Hhh3TVY4fjotY9_0G00">
    <img alt="entity relationship diagram">
</picture>



### <a id="delivery-of-books-to-warehouse">Поставка книг на склад</a>

В данной ИС поставщик формирует поставки, а магазин выбирает те из них, которые он желает приобрести, и определяет склад для поставки книг.

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://www.plantuml.com/plantuml/dpng/RLJDRjD04BxlKupIWqyj1suSKAyy0q6L4m_ngiwkjRicYd2bj0WG4XBSG0W9SUA6J9hGAb7CAupVY8bNgJTj3fQyy_tiDvwHJwsBZIl6cOWWbGc2In5ihftmnw4qmmmPZ5zfcK1kz233iOM9T2d4QfHX0aEZ9nPU8ZfXpwNAOnEF8PCAtJH7qCQbEW16HXVvCvMYaIvMGuJ7GaIed7HJw05zfdbpHNKpuqDi8IvOROJe2DzeGxTqJPKd3BG-FyD4EgdLdj5Nkg45LMosf7NpiRdajpdLNfKNWqnQpY12wv-qeZLLjF2aIMmmrOL527s_C675puqdQPEWWT3b1zNjPHLtye6ZhZpH5dcUIJI7cLlBW3U5lLv-DpCsMj1jyvwMpIMJQre2hRdmxgwbT6hbCCxESZHMgr3-Ywhsdiq1MOQZJXDa-ywfhhTjyiKRJxLE6uIZe-U9TWwJ5qBiUeBZuwUxu4-Wnmbh__KvxHL0JuHJWkDaZzrN_A7UieBxOpRD0Vgg9_njUyOjfwO_pQpvr3e_OctDZV7OkBxfgW10_xPq1kTSdD7VNPo-4jwmzUvcp4bOqWwaUSDlFZjjgDes4Drj39JboKdvUd_n8h-dSC6OleU9A3pkA3x3GQmBzH0sZFkTOzykV1cQXW1BGeKdjDMox5OoQd6uq6tvmIQ5wBs5QNyIOnmF-2nEKINyA_aF ">
    <source media="(prefers-color-scheme: light)" srcset="https://www.plantuml.com/plantuml/png/RLJDRjD04BxlKupIWqyj1suSKAyy0q6L4m_ngiwkjRicYd2bj0WG4XBSG0W9SUA6J9hGAb7CAupVY8bNgJTj3fQyy_tiDvwHJwsBZIl6cOWWbGc2In5ihftmnw4qmmmPZ5zfcK1kz233iOM9T2d4QfHX0aEZ9nPU8ZfXpwNAOnEF8PCAtJH7qCQbEW16HXVvCvMYaIvMGuJ7GaIed7HJw05zfdbpHNKpuqDi8IvOROJe2DzeGxTqJPKd3BG-FyD4EgdLdj5Nkg45LMosf7NpiRdajpdLNfKNWqnQpY12wv-qeZLLjF2aIMmmrOL527s_C675puqdQPEWWT3b1zNjPHLtye6ZhZpH5dcUIJI7cLlBW3U5lLv-DpCsMj1jyvwMpIMJQre2hRdmxgwbT6hbCCxESZHMgr3-Ywhsdiq1MOQZJXDa-ywfhhTjyiKRJxLE6uIZe-U9TWwJ5qBiUeBZuwUxu4-Wnmbh__KvxHL0JuHJWkDaZzrN_A7UieBxOpRD0Vgg9_njUyOjfwO_pQpvr3e_OctDZV7OkBxfgW10_xPq1kTSdD7VNPo-4jwmzUvcp4bOqWwaUSDlFZjjgDes4Drj39JboKdvUd_n8h-dSC6OleU9A3pkA3x3GQmBzH0sZFkTOzykV1cQXW1BGeKdjDMox5OoQd6uq6tvmIQ5wBs5QNyIOnmF-2nEKINyA_aF ">
    <img alt="delivery of books to warehouse diagram">
</picture>

**_Поставщик_**
- Пользователь системы который может создавать и удалять собственные поставки.
- Информация в БД: название поставщика и его юридический адрес, банк поставщика и номер счета в этом банке, ИНН поставщика.

**_Поставка_**
- Содержит перечень книг, их количество и цены. Отправляется поставщиком на склад только после получения подтверждения от магазина.
- Информация в БД: дата поставки, общая стоимость поставки, склад куда будет совершена поставка

**_Книга_**
- Издание книги из БД магазина. Входит в поставку.
- Информация в БД: уникальный идентификатор издания книги

**_Склад_**
- Один из складов магазина, куда будет соверешена поставка.
- Информация в БД: название скалад, адрес склада, его координаты и вместительность

**_Магазин_**
- Пользователь системы, который выбирает необходимые поставки и устанавливает им склад и дату поставки, а также подтверждает получение поставки на складе. Может отклонить поставку.


### Учет книг
...


### Оформление заказа
...


