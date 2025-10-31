#!/bin/bash
# Demo: Interactive widgets (buttons, forms, progress bars)

set -e

echo "=== Next-Gen Terminal: Interactive Widgets Demo ==="
echo
echo "This demo showcases interactive UI components that LLMs can create."
echo
sleep 2

# Create a progress bar
echo "Creating progress bar..."
echo -ne "\e]1339;widget=progress;id=prog1;value=0.0;label=Processing files\a"
sleep 1

# Simulate progress updates
for i in {0..10}; do
    value=$(echo "scale=2; $i / 10" | bc)
    label="Processing files: $((i * 10))%"
    echo -ne "\e]1339;update=widget;id=prog1;value=$value;label=$label\a"
    sleep 0.3
done
echo

# Create buttons
echo
echo "Creating action buttons..."
echo -ne "\e]1339;widget=button;id=btn1;label=Deploy to Production;style=danger;callback=echo 'Deploying...'\a"
echo -ne "\e]1339;widget=button;id=btn2;label=Run Tests;style=success;callback=pytest\a"
echo -ne "\e]1339;widget=button;id=btn3;label=Cancel;style=secondary;callback=echo 'Cancelled'\a"
sleep 1

# Create a form
echo
echo "Creating interactive form..."
echo -ne "\e]1339;widget=form;id=form1;title=Deployment Configuration\a"
echo -ne "\e]1339;widget=form_field;form=form1;name=env;type=select;label=Environment;options=dev,staging,prod;required=true\a"
echo -ne "\e]1339;widget=form_field;form=form1;name=branch;type=text;label=Branch;default=main\a"
echo -ne "\e]1339;widget=form_field;form=form1;name=confirm;type=checkbox;label=I understand this will deploy\a"
sleep 1

# Create a data table
echo
echo "Creating data table..."
echo -ne "\e]1339;widget=table;id=table1;columns=Name,Status,Time;sortable=true\a"
echo -ne "\e]1339;widget=table_row;table=table1;data=Build,✓ Success,2.3s\a"
echo -ne "\e]1339;widget=table_row;table=table1;data=Tests,✓ Success,15.7s\a"
echo -ne "\e]1339;widget=table_row;table=table1;data=Deploy,⏳ Running,--\a"
sleep 1

# Create a chart
echo
echo "Creating chart..."
echo -ne "\e]1339;widget=chart;id=chart1;type=line;title=Response Time (ms)\a"
echo -ne "\e]1339;widget=chart_data;chart=chart1;labels=00:00,00:05,00:10,00:15,00:20;values=120,150,130,140,125\a"
sleep 1

echo
echo "=== Widgets Created ===$"
echo
echo "Interactive widgets allow LLMs to:"
echo "  ✓ Show progress during long operations"
echo "  ✓ Create clickable buttons for actions"
echo "  ✓ Collect input via forms"
echo "  ✓ Display data in sortable tables"
echo "  ✓ Visualize metrics with charts"
echo
echo "This transforms the terminal from read-only to fully interactive!"
echo
