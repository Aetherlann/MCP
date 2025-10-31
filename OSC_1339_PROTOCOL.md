# OSC 1339 Protocol Specification

## Next-Generation Terminal Interface Protocol

OSC 1339 extends the terminal with streaming updates, interactive widgets, flexible layouts, and bidirectional communication.

## Overview

OSC 1339 uses the format:
```
ESC ] 1339 ; <command> ; <parameters> BEL
```

Or with ST terminator:
```
ESC ] 1339 ; <command> ; <parameters> ESC \
```

## Command Categories

### 1. Layout Commands (region management)

#### region=create
Create a new region or split existing one
```bash
echo -ne "\e]1339;region=create;id=region1;bounds=0,0,800,600\a"
```

Parameters:
- `id`: Unique region identifier
- `bounds`: x,y,width,height in pixels or percentage

#### region=split
Split a region horizontally or vertically
```bash
echo -ne "\e]1339;region=split;id=root;direction=vertical;ratio=0.5\a"
```

Parameters:
- `id`: Region to split
- `direction`: `horizontal` or `vertical`
- `ratio`: Split ratio (0.0-1.0)

Returns two new region IDs

#### region=layout
Apply a complex layout specification
```bash
echo -ne "\e]1339;region=layout;spec={\"type\":\"split\",...}\a"
```

Parameters:
- `spec`: JSON layout specification

#### region=content
Set content for a region
```bash
echo -ne "\e]1339;region=content;id=region1;type=terminal\a"
echo -ne "\e]1339;region=content;id=region2;type=gallery;gallery_id=gallery1\a"
```

Parameters:
- `id`: Region ID
- `type`: `terminal`, `gallery`, `widget`, `markdown`, `code`, `tree`, `table`
- Additional type-specific parameters

### 2. Widget Commands

#### widget=button
Create a button widget
```bash
echo -ne "\e]1339;widget=button;id=btn1;label=Click Me;callback=/bin/echo clicked\a"
```

Parameters:
- `id`: Widget ID
- `label`: Button text
- `style`: `primary`, `secondary`, `success`, `danger`, `warning`, `info`
- `callback`: Command or URL to execute on click

#### widget=form
Create a form with multiple fields
```bash
echo -ne "\e]1339;widget=form;id=form1;title=User Info\a"
echo -ne "\e]1339;widget=form_field;form=form1;name=username;type=text;label=Username;required=true\a"
echo -ne "\e]1339;widget=form_field;form=form1;name=email;type=email;label=Email\a"
echo -ne "\e]1339;widget=form_submit;form=form1;callback=/bin/process-form\a"
```

Field types:
- `text`: Single/multiline text
- `number`: Numeric input
- `email`: Email validation
- `password`: Masked input
- `checkbox`: Boolean
- `radio`: Single choice
- `select`: Dropdown
- `date`, `time`: Date/time pickers
- `file`: File upload

#### widget=table
Create a data table
```bash
echo -ne "\e]1339;widget=table;id=table1;columns=Name,Email,Status\a"
echo -ne "\e]1339;widget=table_row;table=table1;data=John,john@example.com,Active\a"
```

Parameters:
- `columns`: Comma-separated column names
- `sortable=true`: Enable sorting
- `filterable=true`: Enable filtering
- `paginated=true;page_size=20`: Enable pagination

#### widget=chart
Create a chart/graph
```bash
echo -ne "\e]1339;widget=chart;id=chart1;type=line;title=Sales\a"
echo -ne "\e]1339;widget=chart_data;chart=chart1;labels=Jan,Feb,Mar;values=100,150,200\a"
```

Chart types:
- `line`: Line chart
- `bar`: Bar chart
- `pie`: Pie chart
- `scatter`: Scatter plot
- `area`: Area chart

#### widget=progress
Create a progress bar
```bash
echo -ne "\e]1339;widget=progress;id=prog1;value=0.0;label=Processing\a"
```

Parameters:
- `value`: 0.0 to 1.0
- `label`: Optional text
- `indeterminate=true`: Spinner mode
- `style`: `bar`, `circle`, `spinner`

#### widget=select
Create a dropdown/select
```bash
echo -ne "\e]1339;widget=select;id=sel1;options=Option A,Option B,Option C\a"
```

Parameters:
- `options`: Comma-separated options
- `multiple=true`: Multi-select
- `searchable=true`: Searchable dropdown

#### widget=tree
Create a tree view
```bash
echo -ne "\e]1339;widget=tree;id=tree1;root={\"id\":\"root\",\"label\":\"Root\",\"children\":[...]}\a"
```

Parameters:
- `root`: JSON tree structure

#### widget=tabs
Create a tabbed interface
```bash
echo -ne "\e]1339;widget=tabs;id=tabs1\a"
echo -ne "\e]1339;widget=tabs_add;tabs=tabs1;id=tab1;label=Code;content=...\a"
echo -ne "\e]1339;widget=tabs_add;tabs=tabs1;id=tab2;label=Output;content=...\a"
```

#### widget=markdown
Render markdown content
```bash
echo -ne "\e]1339;widget=markdown;id=md1;content=$(base64 <<< '# Hello\n**World**')\a"
```

Parameters:
- `content`: Base64-encoded markdown
- `theme`: `light`, `dark`, `github`, `nord`

### 3. Streaming Commands

#### stream=start
Start a streaming context (gallery, table, etc.)
```bash
echo -ne "\e]1339;stream=start;type=gallery;id=gallery1;mode=auto\a"
```

Parameters:
- `type`: `gallery`, `table`, `tree`, `list`
- `id`: Stream identifier
- Additional type-specific parameters

#### stream=add
Add an item to a streaming context
```bash
echo -ne "\e]1339;stream=add;stream=gallery1;title=Image 1\a"
# Followed by image data (Kitty graphics protocol)
```

#### stream=update
Update an existing item in a stream
```bash
echo -ne "\e]1339;stream=update;stream=gallery1;item=item1;title=Updated\a"
```

#### stream=end
Finalize a streaming context
```bash
echo -ne "\e]1339;stream=end;stream=gallery1\a"
```

### 4. Bidirectional Communication

#### callback=register
Register a callback handler
```bash
echo -ne "\e]1339;callback=register;id=handler1;event=click;command=/bin/handle-click\a"
```

Parameters:
- `id`: Handler ID
- `event`: Event type (`click`, `submit`, `change`, etc.)
- `command`: Shell command to execute
- `url`: HTTP URL to POST to

#### callback=url
Set a callback URL for HTTP communication
```bash
echo -ne "\e]1339;callback=url;widget=form1;url=https://api.example.com/submit;method=POST\a"
```

Parameters:
- `widget`: Widget ID
- `url`: HTTP endpoint
- `method`: `GET`, `POST`, `PUT`, `DELETE`
- `headers`: JSON-encoded headers

#### event=send
Terminal sends event back to shell
```
ESC ] 1339 ; event=click ; widget=btn1 ; data={"value":"clicked"} BEL
```

This is sent by the terminal to the controlling process.

### 5. State Management

#### state=save
Save current UI state
```bash
echo -ne "\e]1339;state=save;id=session1\a"
```

#### state=load
Load a previously saved state
```bash
echo -ne "\e]1339;state=load;id=session1\a"
```

#### state=export
Export current state as JSON
```bash
echo -ne "\e]1339;state=export;file=/path/to/state.json\a"
```

### 6. Update Commands

#### update=widget
Update widget properties
```bash
echo -ne "\e]1339;update=widget;id=prog1;value=0.5;label=50%% complete\a"
```

#### update=table
Update table data
```bash
echo -ne "\e]1339;update=table;id=table1;row=0;col=2;value=Updated\a"
```

#### update=chart
Update chart data
```bash
echo -ne "\e]1339;update=chart;id=chart1;dataset=0;add=250\a"
```

### 7. Query Commands

#### query=regions
Query current region layout
```bash
echo -ne "\e]1339;query=regions\a"
```

Response sent back via OSC 1339:
```
ESC ] 1339 ; response=regions ; data={"root":{...}} BEL
```

#### query=widgets
List all widgets
```bash
echo -ne "\e]1339;query=widgets\a"
```

#### query=state
Get current UI state
```bash
echo -ne "\e]1339;query=state\a"
```

## Complete Examples

### Example 1: Split Terminal with Code and Output

```bash
# Split terminal vertically (50/50)
echo -ne "\e]1339;region=split;id=root;direction=vertical;ratio=0.5\a"

# Left side: code editor
echo -ne "\e]1339;region=content;id=left;type=code;language=python\a"
cat script.py

# Right side: output
echo -ne "\e]1339;region=content;id=right;type=terminal\a"
python script.py
```

### Example 2: Interactive Form

```bash
# Create form
echo -ne "\e]1339;widget=form;id=deploy-form;title=Deploy Configuration\a"

# Add fields
echo -ne "\e]1339;widget=form_field;form=deploy-form;name=env;type=select;label=Environment;options=dev,staging,prod;required=true\a"
echo -ne "\e]1339;widget=form_field;form=deploy-form;name=branch;type=text;label=Branch;default=main\a"
echo -ne "\e]1339;widget=form_field;form=deploy-form;name=confirm;type=checkbox;label=I understand this will deploy to production\a"

# Set callback
echo -ne "\e]1339;widget=form_submit;form=deploy-form;callback=/usr/local/bin/deploy\a"

# When user submits, terminal calls: /usr/local/bin/deploy --env=prod --branch=main --confirm=true
```

### Example 3: Streaming Data Table

```bash
# Start streaming table
echo -ne "\e]1339;stream=start;type=table;id=logs;columns=Time,Level,Message\a"

# Stream log entries as they arrive
tail -f /var/log/app.log | while read line; do
    time=$(echo "$line" | cut -d' ' -f1)
    level=$(echo "$line" | cut -d' ' -f2)
    msg=$(echo "$line" | cut -d' ' -f3-)
    echo -ne "\e]1339;stream=add;stream=logs;data=$time,$level,$msg\a"
done
```

### Example 4: Dashboard Layout

```bash
# Create complex dashboard layout
cat <<'EOF' | base64 -d | xargs -0 echo -ne
\e]1339;region=layout;spec={
  "type": "split",
  "direction": "horizontal",
  "ratio": 0.3,
  "first": {
    "type": "single",
    "content": {"type": "terminal"}
  },
  "second": {
    "type": "split",
    "direction": "vertical",
    "ratio": 0.5,
    "first": {
      "type": "single",
      "content": {"type": "widget", "widget_id": "chart1"}
    },
    "second": {
      "type": "single",
      "content": {"type": "gallery", "gallery_id": "gallery1"}
    }
  }
}\a
EOF

# Now you have:
# - Top 30%: Terminal
# - Bottom left 35%: Chart
# - Bottom right 35%: Gallery
```

### Example 5: AI Assistant with Progress

```bash
# LLM starts analysis
echo "Analyzing codebase..."
echo -ne "\e]1339;widget=progress;id=analysis;value=0.0;label=Scanning files\a"

# Update as analysis progresses
echo -ne "\e]1339;update=widget;id=analysis;value=0.25;label=Parsing syntax\a"
echo -ne "\e]1339;update=widget;id=analysis;value=0.50;label=Running linters\a"
echo -ne "\e]1339;update=widget;id=analysis;value=0.75;label=Generating report\a"

# Show results in gallery
echo -ne "\e]1339;stream=start;type=gallery;id=results;mode=grid\a"
# Stream results...
echo -ne "\e]1339;stream=end;stream=results\a"

# Complete
echo -ne "\e]1339;update=widget;id=analysis;value=1.0;label=Complete\a"
```

## Protocol Features

### Streaming Support
All collection types (galleries, tables, trees, lists) support streaming:
- Use `stream=start` to begin
- Use `stream=add` to add items incrementally
- Use `stream=update` to modify items
- Use `stream=end` to finalize

### Bidirectional Communication
- Widgets can trigger callbacks (shell commands or HTTP URLs)
- Terminal sends events back to shell via OSC 1339
- Supports request/response patterns

### State Management
- Save/load UI state for session persistence
- Export state as JSON for sharing
- Undo/redo support (planned)

### Flexible Layouts
- Split terminal into arbitrary regions
- Apply complex layouts via JSON specs
- Resize and reflow automatically
- Support for floating/overlay regions (planned)

## Implementation Notes

### Parser Integration
OSC 1339 commands are parsed by `ht-vt` and generate `VtToken::NextGen(NextGenCommand)` tokens.

### Layout Manager
The `ht-layout` crate manages region splitting and layout calculations.

### Widget System
The `ht-widgets` crate provides all interactive components.

### Event Loop
Events flow: User Action → Widget → Event Handler → Callback → Shell/URL

## Compatibility

OSC 1339 is designed to coexist with:
- OSC 1338 (Gallery Protocol)
- Kitty Graphics Protocol
- iTerm2 Inline Images
- Standard VT/ANSI sequences

Terminals that don't support OSC 1339 will ignore these sequences safely.

## See Also

- [OSC 1338 Gallery Protocol](LLM_GALLERY_PROTOCOL.md)
- [Layout Manager Documentation](crates/ht-layout/README.md)
- [Widget System Documentation](crates/ht-widgets/README.md)
