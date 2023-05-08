# Local Image Hosting

A simple and efficient image hosting program based on local disk storage, built using the async-std and tide frameworks in Rust. 

This program allows you to download images from specified URLs to your local machine, host the images on a local server, and delete them when needed, making it perfect for personal use or small-scale projects.

## Features

**hosting**
- Host images on a local server
- Get storage information (total and free space)

**upload**
- Download images from specified URLs to local disk storage
- Delete images from local disk storage

## Usage

The program has two main components, `hosting.rs` and `upload.rs`, which should be run simultaneously. `hosting.rs` serves as the image hosting server, while `upload.rs` handles image uploads and deletions.

### Configuration

Before running the program, set the `IMAGEHOSTING_PATH` environment variable to specify the path where images will be stored. You can do this by adding the following line to your `.env` file:

```
IMAGEHOSTING_PATH=/path/to/your/image/storage/folder
```

### Endpoints

- `/images`: Serves the images from the specified storage folder.
- `/v2/hosting`: Handles image uploads and deletions.
  - POST: Download an image from a specified URL.
  - DELETE: Delete an image.
- `/v2/storage`: Get storage information (total and free space).


## License

This project is licensed under the MIT License.
