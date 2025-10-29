# LLM Gallery Protocol - Integration Guide

## Overview

This protocol enables LLMs (like Claude, GPT, etc.) to create beautiful, organized galleries in Hyper Terminal. Instead of scattering inline images throughout the output, LLMs can group related content into intelligent, visually stunning galleries.

## Why This Matters

**Before** (Traditional):
```
Here are some design options:

[Inline Image 1]
This is option A with a blue theme.

[Inline Image 2]
This is option B with a red theme.

[Inline Image 3]
This is option C with a green theme.
```

**After** (Gallery):
```
Here are some design options:

┌────────────── Design Options (3 items) ──────────────┐
│  ┌──────┐  ┌──────┐  ┌──────┐                       │
│  │Opt A │  │Opt B │  │Opt C │                       │
│  │Blue  │  │Red   │  │Green │                       │
│  └──────┘  └──────┘  └──────┘                       │
│  [💾][🔍] [💾][🔍] [💾][🔍]                           │
└───────────────────────────────────────────────────────┘
```

## Protocol Specification

### 1. Gallery Control Sequences

#### Start Gallery
```bash
ESC ] 1338 ; gallery=start ; id=<unique_id> ; [options] BEL
```

**Parameters**:
- `id` (required): Unique identifier for this gallery
- `mode` (optional): Display mode (grid, masonry, filmstrip, comparison, deck, auto)
- `title` (optional): Gallery title
- `columns` (optional): Number of columns for grid mode
- `spacing` (optional): Spacing between cards (default: 12)

**Example**:
```bash
echo "\e]1338;gallery=start;id=g1;mode=grid;title=Logo Concepts\a"
```

#### End Gallery
```bash
ESC ] 1338 ; gallery=end ; id=<gallery_id> BEL
```

**Example**:
```bash
echo "\e]1338;gallery=end;id=g1\a"
```

### 2. Media Item Commands

#### Add Image to Gallery
```bash
ESC ] 1338 ; media=item ; gallery=<id> ; [metadata] BEL
<Image data via Kitty protocol>
```

**Metadata Parameters**:
- `title`: Item title
- `desc`: Description
- `tags`: Comma-separated tags
- `author`: Creator name
- `source`: Source identifier

**Example**:
```bash
# Send metadata
echo "\e]1338;media=item;gallery=g1;title=Logo A;desc=Minimalist design;tags=minimal,modern\a"

# Send image via Kitty protocol
printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < logo_a.png)"
```

#### Add File to Gallery
```bash
ESC ] 1338 ; file=item ; gallery=<id> ; path=<path> ; [metadata] BEL
```

**Example**:
```bash
echo "\e]1338;file=item;gallery=g1;path=src/main.rs;title=Main Entry;desc=Application entry point\a"
```

## Usage Examples

### Example 1: Image Gallery (Grid Mode)

**Scenario**: LLM generates 4 logo concepts

```bash
#!/bin/bash

# Start gallery
echo "\e]1338;gallery=start;id=logos;mode=grid;title=Logo Concepts\a"

# Add images
for i in {1..4}; do
    echo "\e]1338;media=item;gallery=logos;title=Concept $i;desc=Design option $i\a"
    printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < concept_$i.png)"
done

# End gallery
echo "\e]1338;gallery=end;id=logos\a"
```

**Result**: Beautiful 2x2 grid of logo concepts with titles and quick actions.

### Example 2: Comparison Mode

**Scenario**: LLM shows before/after comparison

```bash
#!/bin/bash

# Start comparison gallery
echo "\e]1338;gallery=start;id=compare;mode=comparison;title=Before vs After\a"

# Before image
echo "\e]1338;media=item;gallery=compare;title=Before;desc=Original image\a"
printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < before.png)"

# After image
echo "\e]1338;media=item;gallery=compare;title=After;desc=Enhanced version\a"
printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < after.png)"

# End gallery
echo "\e]1338;gallery=end;id=compare\a"
```

**Result**: Side-by-side comparison with synchronized zoom and slider.

### Example 3: File Explorer Mode

**Scenario**: LLM generates a project structure

```bash
#!/bin/bash

# Start file explorer
echo "\e]1338;gallery=start;id=project;mode=file_explorer;title=Generated Project\a"

# Add files
files=(
    "src/main.rs:Main Entry:Rust"
    "src/lib.rs:Library Root:Rust"
    "Cargo.toml:Package Config:TOML"
    "README.md:Documentation:Markdown"
)

for file in "${files[@]}"; do
    IFS=: read -r path title lang <<< "$file"
    echo "\e]1338;file=item;gallery=project;path=$path;title=$title;lang=$lang\a"
done

# End gallery
echo "\e]1338;gallery=end;id=project\a"
```

**Result**: Tree view with file previews and save-all option.

### Example 4: Filmstrip Mode

**Scenario**: LLM shows step-by-step process

```bash
#!/bin/bash

# Start filmstrip
echo "\e]1338;gallery=start;id=steps;mode=filmstrip;title=Image Processing Pipeline\a"

# Add steps
for i in {1..6}; do
    echo "\e]1338;media=item;gallery=steps;title=Step $i;desc=Processing stage $i\a"
    printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < step_$i.png)"
done

# End gallery
echo "\e]1338;gallery=end;id=steps\a"
```

**Result**: Horizontal scrollable filmstrip with navigation arrows.

## Claude Integration Examples

### Example 1: Design Generation

**User**: "Generate 3 logo concepts for my tech startup"

**Claude Response**:
```
I'll create 3 logo concepts for your tech startup, exploring different design styles.

[Initializes gallery]
<Gallery control sequence: start grid gallery>

Generating Concept 1: Geometric Abstraction
<Creates logo image>
<Sends metadata + image>

Generating Concept 2: Minimalist Wordmark
<Creates logo image>
<Sends metadata + image>

Generating Concept 3: Symbolic Icon
<Creates logo image>
<Sends metadata + image>

<Gallery control sequence: end>

All 3 concepts are displayed above in an interactive grid. You can:
- Click any logo to view full-screen
- Save individual concepts with the 💾 button
- Copy to clipboard with 📋
- Compare them side-by-side

Which direction resonates with you?
```

**Terminal Display**:
```
┌────────── Logo Concepts (3 items) ──────────┐
│                                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐ │
│  │Concept 1 │  │Concept 2 │  │Concept 3 │ │
│  │Geometric │  │Minimalist│  │Symbolic  │ │
│  │          │  │          │  │          │ │
│  │[Logo Img]│  │[Logo Img]│  │[Logo Img]│ │
│  │          │  │          │  │          │ │
│  │Abstract  │  │Clean     │  │Icon-based│ │
│  │shapes    │  │wordmark  │  │design    │ │
│  │          │  │          │  │          │ │
│  │PNG·245KB │  │PNG·234KB │  │PNG·256KB │ │
│  │          │  │          │  │          │ │
│  │[💾][🔍][📋]│  │[💾][🔍][📋]│  │[💾][🔍][📋]│ │
│  └──────────┘  └──────────┘  └──────────┘ │
│                                              │
└──────────────────────────────────────────────┘
```

### Example 2: Code Scaffolding

**User**: "Create a React component library structure"

**Claude Response**:
```
I'll scaffold a complete React component library for you.

[Initializes file explorer]
<Gallery control sequence: start file_explorer>

Creating component library structure...

<Sends file metadata for each file>
- package.json
- tsconfig.json
- src/index.ts
- src/components/Button.tsx
- src/components/Input.tsx
- src/components/Card.tsx
- src/styles/theme.css
- README.md

<Gallery control sequence: end>

Your component library is ready! The file explorer above shows:
- 📁 Full directory structure
- 📄 All generated files with syntax highlighting
- 💾 Save all files with one click
- ✏️  Edit any file inline

Click "Save All" to save to ~/Downloads/component-library/
```

### Example 3: Data Visualization

**User**: "Analyze this CSV and show trends"

**Claude Response**:
```
I've analyzed your data and created visualizations:

[Initializes deck gallery]
<Gallery control sequence: start deck>

Card 1: Sales Trends
<Generates chart image>
<Sends with metadata>

Card 2: Regional Performance
<Generates map visualization>
<Sends with metadata>

Card 3: Customer Segments
<Generates pie chart>
<Sends with metadata>

Card 4: Forecast
<Generates projection chart>
<Sends with metadata>

<Gallery control sequence: end>

Navigate through the cards above with arrow keys or [Next ▶] button.
Each card includes:
- Interactive chart
- Key insights
- Raw data table
- Export options
```

## Smart Mode Detection

When using `mode=auto`, the gallery automatically selects the best display mode:

**Detection Rules**:
```rust
fn auto_detect_mode(items: &[MediaItem]) -> GalleryMode {
    if items.len() == 2 {
        GalleryMode::Comparison  // Perfect for A/B comparisons
    } else if items.len() <= 4 && similar_aspects(items) {
        GalleryMode::Grid  // Clean grid for few similar items
    } else if is_sequential(items) {
        GalleryMode::Filmstrip  // Steps, iterations, timeline
    } else if all_files(items) {
        GalleryMode::FileExplorer  // File/code generation
    } else {
        GalleryMode::Masonry  // Flexible for mixed content
    }
}
```

**Hints for Smart Detection**:
- **Sequential numbering** in titles → Filmstrip
- **"before"/"after"** keywords → Comparison
- **File paths** → File Explorer
- **Similar dimensions** → Grid
- **Mixed content** → Masonry

## Best Practices

### 1. Always Provide Metadata

**Good**:
```bash
echo "\e]1338;media=item;gallery=g1;title=Logo A;desc=Minimalist design with blue accent;tags=minimal,modern,blue\a"
```

**Bad**:
```bash
echo "\e]1338;media=item;gallery=g1\a"
```

Metadata makes content searchable, organized, and professional.

### 2. Use Descriptive Gallery IDs

**Good**: `gallery=design_concepts_v2`
**Bad**: `gallery=g1`

Descriptive IDs help with debugging and logging.

### 3. Choose Appropriate Modes

| Content Type | Best Mode |
|--------------|-----------|
| Few similar images (2-6) | Grid |
| Many varied images | Masonry |
| Before/after, A vs B | Comparison |
| Sequential steps | Filmstrip |
| Comprehensive report | Deck |
| Files/code | File Explorer |
| Unsure | Auto |

### 4. Group Related Content

**Good**: One gallery per logical group
```
Gallery 1: "Logo Concepts" (3 logos)
Gallery 2: "Business Card Designs" (2 designs)
```

**Bad**: Everything in one huge gallery
```
Gallery 1: "Designs" (5 logos + 2 cards + 3 fonts + ...)
```

### 5. Add Context in Descriptions

**Good**:
```
title=Option A - Bold
desc=High-contrast design optimized for small sizes. Works well in both light and dark themes.
tags=bold,contrast,responsive
```

**Bad**:
```
title=A
desc=Design
```

## Error Handling

### Missing Gallery
If media is sent without starting a gallery:
- Terminal creates implicit gallery with `mode=auto`
- ID is auto-generated: `auto_gallery_<timestamp>`

### Invalid Parameters
- Unknown parameters are ignored
- Invalid modes fall back to `auto`
- Malformed sequences are logged but don't crash

### Recovery
```bash
# Force close any open gallery
echo "\e]1338;gallery=close_all\a"

# Query gallery state
echo "\e]1338;gallery=status\a"
```

## Advanced Features

### Annotations

Add markers to images:
```bash
echo "\e]1338;annotation=add;gallery=g1;item=0;type=arrow;from=100,100;to=200,200;label=Important\a"
```

### Groups Within Galleries

Organize items into sub-groups:
```bash
echo "\e]1338;group=start;gallery=g1;name=Primary Options\a"
# Items...
echo "\e]1338;group=end;gallery=g1\a"
```

### Custom Actions

Define custom actions for cards:
```bash
echo "\e]1338;action=define;id=custom1;label=Upscale;icon=⬆️;cmd=upscale %path%\a"
echo "\e]1338;media=item;gallery=g1;actions=save,copy,custom1\a"
```

## Performance Considerations

**Optimal Gallery Sizes**:
- Grid: 2-12 items ideal
- Masonry: Up to 50 items
- Filmstrip: 3-20 items
- Comparison: 2-4 items max
- Deck: 3-10 cards
- File Explorer: Any size (lazy loaded)

**Large Collections**:
```bash
# For 100+ items, use pagination
echo "\e]1338;gallery=start;id=g1;page=1;per_page=20\a"
# Items 1-20...
echo "\e]1338;gallery=end;id=g1\a"
```

## Testing

### Test Script

```bash
#!/bin/bash
# test-gallery.sh - Test all gallery modes

echo "Testing Gallery Modes..."

# Test 1: Grid
echo "\e]1338;gallery=start;id=test1;mode=grid;title=Grid Test\a"
for i in {1..4}; do
    echo "\e]1338;media=item;gallery=test1;title=Item $i\a"
    printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < assets/tiny-red.png)"
done
echo "\e]1338;gallery=end;id=test1\a"

echo ""
echo "Grid gallery displayed above. Press Enter for next test..."
read

# Test 2: Comparison
echo "\e]1338;gallery=start;id=test2;mode=comparison;title=Comparison Test\a"
for i in {1..2}; do
    echo "\e]1338;media=item;gallery=test2;title=Option $i\a"
    printf '\e_Gf=100,a=T,t=d;%s\e\\' "$(base64 < assets/tiny-red.png)"
done
echo "\e]1338;gallery=end;id=test2\a"

# More tests...
```

## Future Extensions

Planned features:
- Video galleries with play controls
- 3D model viewers (glTF/GLB)
- PDF page galleries
- Audio waveform visualizations
- Live preview mode (updates as generated)
- Collaborative galleries (multi-user)
- Gallery templates (predefined styles)

## Summary

The LLM Gallery Protocol enables:
- ✅ Beautiful, organized content presentation
- ✅ Intelligent auto-layout
- ✅ Rich metadata and descriptions
- ✅ Interactive features (zoom, save, copy)
- ✅ Professional appearance by default
- ✅ Simple escape sequence API
- ✅ Backward compatible (degrades gracefully)

**For LLM developers**: Use this protocol to make your outputs shine.
**For terminal users**: Enjoy gorgeous, organized results from AI assistants.

---

**Ready to create world-class galleries? Let's make every AI interaction beautiful.** ✨
