param(
    [string]$VideoPath = "",
    [int]$ParentPid = 0
)

Add-Type -AssemblyName PresentationCore, PresentationFramework, WindowsBase, System.Drawing

if ([string]::IsNullOrWhiteSpace($VideoPath) -or -not (Test-Path $VideoPath)) {
    # Check default locations
    $candidates = @(
        "$PSScriptRoot\background.mp4",
        "$PSScriptRoot\assets\background.mp4",
        "..\itzamna-desktop\assets\background.mp4"
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) {
            $VideoPath = (Resolve-Path $c).Path
            break
        }
    }
}

if (-not (Test-Path $VideoPath)) {
    exit 0
}

$xaml = @"
<Window xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
        xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
        Title="Itzamná Consecration Chamber"
        Width="460" Height="260"
        WindowStyle="None"
        AllowsTransparency="True"
        Background="Transparent"
        WindowStartupLocation="Manual"
        Topmost="False"
        ShowInTaskbar="False">
    <Border CornerRadius="12" BorderBrush="#FFD700" BorderThickness="1.5" Background="#101219" ClipToBounds="True">
        <Grid>
            <MediaElement Name="Player" LoadedBehavior="Manual" Stretch="UniformToFill" />
            <Border Background="#440A0B0F" />
            
            <!-- Top Controls -->
            <Grid VerticalAlignment="Top" Margin="12,10,12,0">
                <StackPanel Orientation="Horizontal" HorizontalAlignment="Left">
                    <TextBlock Text="🔱" FontSize="12" Foreground="#FFD700" VerticalAlignment="Center"/>
                    <TextBlock Text=" ITZAMNÁ " FontSize="11" FontWeight="Bold" Foreground="#FFD700" VerticalAlignment="Center"/>
                    <TextBlock Text="· CONSECRATION RITUAL" FontSize="10" Foreground="#A6ADC0" VerticalAlignment="Center"/>
                </StackPanel>
                <Button Name="BtnClose" Content="✕" Width="20" Height="20"
                        Background="#2A2F3D" Foreground="#A6ADC0" BorderThickness="0"
                        HorizontalAlignment="Right" Cursor="Hand" />
            </Grid>

            <!-- Bottom Cosmic Inscription -->
            <Border VerticalAlignment="Bottom" Background="#DD101219" Padding="10,6" BorderBrush="#2A2F3D" BorderThickness="0,1,0,0">
                <StackPanel HorizontalAlignment="Center">
                    <TextBlock Text="Carving the Stelae &amp; Weaving the Yaxché Loom..." 
                               Foreground="#00F299" FontSize="10" FontWeight="SemiBold" HorizontalAlignment="Center"/>
                    <TextBlock Text="· ☀️ · 🌿 · ⚖️ · 🔮 · 🩸 · 🏛️ · ⚡ ·" 
                               Foreground="#666E80" FontSize="9" HorizontalAlignment="Center" Margin="0,2,0,0"/>
                </StackPanel>
            </Border>
        </Grid>
    </Border>
</Window>
"@

$reader = [System.Xml.XmlReader]::Create([System.IO.StringReader]::new($xaml))
$win = [System.Windows.Markup.XamlReader]::Load($reader)
$player = $win.FindName("Player")
$btnClose = $win.FindName("BtnClose")

# Position in the top-right corner of the primary screen, away from center wizard
$workArea = [System.Windows.SystemParameters]::WorkArea
$win.Left = $workArea.Right - 460 - 24
$win.Top = $workArea.Top + 24

$player.Source = [System.Uri]::new($VideoPath)
$player.add_MediaEnded({ 
    $player.Position = [System.TimeSpan]::Zero
    $player.Play() 
})

$win.add_Loaded({ 
    $player.Play() 
})

# Allow dragging the window anywhere
$win.add_MouseLeftButtonDown({
    $win.DragMove()
})

$btnClose.add_Click({
    $win.Close()
})

# Watch parent process if supplied
if ($ParentPid -gt 0) {
    $watchTimer = New-Object System.Windows.Threading.DispatcherTimer
    $watchTimer.Interval = [System.TimeSpan]::FromMilliseconds(500)
    $watchTimer.add_Tick({
        $p = Get-Process -Id $ParentPid -ErrorAction SilentlyContinue
        if ($null -eq $p -or $p.HasExited) {
            $watchTimer.Stop()
            $win.Close()
        }
    })
    $watchTimer.Start()
}

$win.ShowDialog() | Out-Null
