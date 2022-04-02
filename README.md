# jsonFetcher

A Programm that accesses a Collection in a MongoDB Database, and for every item in the collection it looks for the field "uri".
If there is the uri field it then tries to get the json Doc which is behind the uri, and append that json do to the item in the Database.
