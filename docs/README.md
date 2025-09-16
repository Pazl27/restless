# Restless Documentation

This directory contains documentation assets for the Restless project.

## Contents

- `demo.gif` - Animated demonstration of Restless in action (coming soon)
- `main-interface.png` - Screenshot of the main interface (coming soon)
- `response-viewer.png` - Screenshot of the response viewer (coming soon)
- `help-screen.png` - Screenshot of the help screen (coming soon)

## Creating Screenshots

To create screenshots for documentation:

1. **Terminal Setup**:
   - Use a terminal with at least 120x40 characters for best quality
   - Use a clean, readable terminal theme (dark background recommended)
   - Ensure good contrast between text and background

2. **Content Guidelines**:
   - Use realistic API endpoints (e.g., JSONPlaceholder, httpbin.org)
   - Show both successful requests and formatted JSON responses
   - Include examples of headers and parameters
   - Demonstrate error states when appropriate

3. **Screenshot Tools**:
   - **macOS**: Use built-in Screenshot app or `screencapture`
   - **Linux**: Use `gnome-screenshot`, `flameshot`, or `scrot`
   - **Windows**: Use Snipping Tool or PowerShell's `Add-Type`

4. **GIF Creation**:
   - Use `asciinema` to record terminal sessions
   - Convert to GIF using `agg` or `asciicast2gif`
   - Keep file size reasonable (< 5MB for GitHub)

## Example Demo Scenarios

### Basic GET Request
1. Start Restless
2. Enter URL: `https://jsonplaceholder.typicode.com/posts/1`
3. Send request
4. Show formatted JSON response

### POST Request with Body
1. Set method to POST
2. Enter URL: `https://httpbin.org/post`
3. Add Content-Type header: `application/json`
4. Add JSON body: `{"title": "Test Post", "body": "This is a test"}`
5. Send request and show response

### Multiple Tabs
1. Show multiple tabs with different requests
2. Demonstrate tab switching
3. Show different request types in each tab

### Help Screen
1. Press `?` to show help
2. Scroll through help content
3. Demonstrate keyboard shortcuts

## File Naming Convention

- Use descriptive names: `main-interface-dark-theme.png`
- Include dimensions in filename if multiple sizes: `demo-800x600.png`
- Use lowercase with hyphens for consistency
- Add version numbers for updates: `demo-v2.gif`

## Optimization

- **Images**: Optimize PNG files with tools like `optipng` or `pngcrush`
- **GIFs**: Keep frame rate reasonable (10-15 fps) and optimize with `gifsicle`
- **File Size**: Target < 2MB for images, < 5MB for GIFs

## Contributing Screenshots

When contributing documentation images:

1. Follow the guidelines above
2. Test images on different backgrounds (GitHub light/dark themes)
3. Ensure text is readable at various zoom levels
4. Include alt text descriptions in markdown
5. Consider accessibility (color contrast, text size)

---

**Note**: This directory currently contains placeholder documentation. Screenshots and demo files will be added as the project develops.