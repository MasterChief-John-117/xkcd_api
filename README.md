# XKCD Search API Design Documentation

### Purpose
Allow searching xkcd comics by title, alt text, or transcript

### Endpoints
* `/` should return info about the latest comic
* `/id/:id` should return info about a comic by ID, or an HTTP 404 response if the ID is not found
* `/title/:query` should normalize the query input and return all comics with a title that match the query
* `/alt/:query` should normalize the query input and return all comics with an alt text that match the query
* `/transcript/:query` should normalize the query input and return all comics with a transcript that match the query
* `/search/:query` should normalize the query input and return all comics with a title OR alt text OR transcript that match the query string, without any duplicates.

# Return Data Format
Data should be returned to the user in a JSON array. Each object in the returned array should contain the following fields of (type): title (string), num (int), alt (string), transcript (string), img (string), year (int), month (int), day (int). They should be ordered by the num field. 

### Text Normalization
In order to increase the likelyhood of getting the desired comic, all text retrieved from the xkcd API should be stored in both the original format and a "normalized" format. This "normalized" format should be created by taking the original text, converting it to lowercase, and removing any characters that are not letters, numbers, or spaces. This normalization should also be run on any text queries the API recieves.

### Data Storage
In order to prevent spamming the xkcd API with requests, all retrieved data should be stored persistently. In order to promote portability, SQLite will be used. However, because full text search support for sqlite requires additional configuration, all data should be loaded into memory when the search API is started, and queries should be run against the in-memory data. Given the size of the data that will be stored, keeping it in memory is acceptable.

### Notes
* The xkcd API returns a `safe_title` and `title` field. The search API should retrieve the safe_title and store it as the cannonical title.
* Comic 404 is not a comic, but instead returns the 404 page. How the search API handles this is undefined