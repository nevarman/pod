# POD - Picture of the Day

POD is a cmd line tool that sets wallpaper background from providers:
- NASA APOD: https://apod.nasa.gov/apod/astropix.html (need API key)
- Bing

# Features

- Can resize the picture. This is recommended to fit your desktop resolution.
- Can modify the picture to show metadata such as the title and explanation from NASA APOD.

# Usage

Use ``pod -h`` to see available commands.

### For NASA

Get your api key at: https://api.nasa.gov/

```
pod --add-metadata true --metadata-font "Fira Code" --metadata-font-size 18 --fit-to-screen-size true --width 2560 --hg 1440 nasa --nasa-api-key=<your api key>
```

### For Bing

```
pod --add-metadata true --metadata-font "Fira Code" --metadata-font-size 18 --fit-to-screen-size true --width 2560 --hg 1440 bing
```

You can run the command from task scheduler or as a systemd service to change your wallpaper daily.

Picture is saved next to installation location.