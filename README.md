# jsonFetcher

A Program that accesses a Collection in a MongoDB Database, and for every item in the collection it looks for the field "uri".
If there is the uri field it then tries to get the json Doc which is behind the uri, and append that json doc to the item in the Database.
Currently appending the data to the item doesnt work, it only prints the fetched data.
