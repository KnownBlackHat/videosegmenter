# 🎥 Video Segmenter

A simple and efficient Rust implementation that leverages `ffmpeg` to split videos into smaller chunks based on your specified size.

## 📥 Installation

Make sure you have [FFmpeg](https://ffmpeg.org/download.html) installed on your system.

## 🚀 Usage

Easily fragment your video into smaller segments with the following command:

```bash
videosegmenter <VIDEO_PATH> <OUTPUT_DIRECTORY> <FRAG_SIZE_MB>
```

### Example

Imagine you have a 100MB video file, but you need to upload it to a platform with a maximum file size limit of 25MB (such as Discord). With `videosegmenter`, you can effortlessly split your video into 25MB chunks:

```bash
videosegmenter myvideo.mp4 output 25
```

## 💡 Why Use This Tool?

While `ffmpeg` is a powerful tool for handling media files, it **doesn't support fragmenting video files by size**. That's where `videosegmenter` comes in! This tool handles the calculations and segmentation for you, saving you the hassle of manually determining segment sizes for multiple video files.

## 🔧 Features

- ⚡ **Quick and Easy**: Just provide the video path, output directory, and desired fragment size.
- 📏 **Size-based Fragmentation**: Splits videos into chunks based on the size you specify.
- 🛠️ **Built on Rust**: Efficient and reliable performance.
