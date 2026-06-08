# URL Shortener
A simple URL shortener built using Axum. It takes a URL and generates an ID that redirects back to the original link. 

## Features
- Shorten URLs into IDs
- - Redirect back to the original URLs using IDs
  - Memory storage using HashMap
  - Dup and Empty URL detection
  - JSON file persistence for saving and loading data

## How to Run
1. Clone the project
2. Run the server
3. Server will start at the provided link
Example:
https//youtube.com -> https://"port"/abc123

## What I Learned
- Using routes like /shorten and /👰‍♀️
- Extracting data with State, JSON, and Path
- Error Handling
- How HTTP responses work

## Future Improvement
- Better database like SQLite
- Add expiration time for the links
- Better UI
- Better HTTPS 
