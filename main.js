const { app, BrowserWindow, ipcMain, globalShortcut } = require('electron');
const path = require('path');

// Audio control module - using loudness as a base (can be extended for per-app control)
let loudness;
try {
  loudness = require('loudness');
} catch (error) {
  console.warn('Loudness module not available, using mock functions');
  // Mock functions for development/testing
  loudness = {
    getVolume: () => Promise.resolve(50),
    setVolume: (vol) => Promise.resolve(vol),
    getMuted: () => Promise.resolve(false),
    setMuted: (muted) => Promise.resolve(muted)
  };
}

let mainWindow;

// Audio sources configuration
const audioSources = {
  music: { name: 'Music Player', volume: 50, keyIncrease: 'F13', keyDecrease: 'F14' },
  browser: { name: 'Browser', volume: 50, keyIncrease: 'F15', keyDecrease: 'F16' },
  system: { name: 'System Sounds', volume: 50, keyIncrease: 'F17', keyDecrease: 'F18' },
  game: { name: 'Games', volume: 50, keyIncrease: 'F19', keyDecrease: 'F20' }
};

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 900,
    height: 700,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false
    },
    icon: path.join(__dirname, 'assets/icon.png'), // Optional icon
    titleBarStyle: 'default',
    resizable: true,
    minWidth: 600,
    minHeight: 500
  });

  mainWindow.loadFile('index.html');

  // Register global shortcuts for F13-F24
  registerGlobalShortcuts();

  mainWindow.on('closed', () => {
    mainWindow = null;
  });

  // Send initial audio sources to renderer
  mainWindow.webContents.once('dom-ready', () => {
    mainWindow.webContents.send('audio-sources', audioSources);
  });
}

function registerGlobalShortcuts() {
  try {
    // Register shortcuts for each audio source
    Object.keys(audioSources).forEach(sourceId => {
      const source = audioSources[sourceId];
      
      // Decrease volume shortcut
      if (source.keyDecrease) {
        globalShortcut.register(source.keyDecrease, () => {
          adjustVolumeByShortcut(sourceId, -5);
        });
      }
      
      // Increase volume shortcut
      if (source.keyIncrease) {
        globalShortcut.register(source.keyIncrease, () => {
          adjustVolumeByShortcut(sourceId, 5);
        });
      }
    });

    // Additional shortcuts
    globalShortcut.register('F21', () => {
      // Mute/Unmute all
      toggleMuteAll();
    });

    globalShortcut.register('F22', () => {
      // Reset all volumes to 50%
      resetAllVolumes();
    });

    console.log('Global shortcuts registered successfully');
  } catch (error) {
    console.error('Error registering global shortcuts:', error);
  }
}

async function adjustVolumeByShortcut(sourceId, change) {
  try {
    if (!audioSources[sourceId]) return;
    
    const currentVolume = audioSources[sourceId].volume;
    const newVolume = Math.max(0, Math.min(100, currentVolume + change));
    
    audioSources[sourceId].volume = newVolume;
    
    // For now, adjust master volume (can be extended for per-app control)
    await loudness.setVolume(newVolume);
    
    if (mainWindow) {
      mainWindow.webContents.send('volume-update', { 
        source: sourceId, 
        volume: newVolume,
        fromShortcut: true
      });
    }
    
    console.log(`${audioSources[sourceId].name} volume adjusted to ${newVolume}% via shortcut`);
  } catch (error) {
    console.error(`Error adjusting volume for ${sourceId}:`, error);
    if (mainWindow) {
      mainWindow.webContents.send('error', `Error adjusting ${audioSources[sourceId]?.name || sourceId} volume`);
    }
  }
}

async function toggleMuteAll() {
  try {
    const isMuted = await loudness.getMuted();
    await loudness.setMuted(!isMuted);
    
    if (mainWindow) {
      mainWindow.webContents.send('mute-update', { muted: !isMuted });
    }
    
    console.log(`Audio ${!isMuted ? 'muted' : 'unmuted'}`);
  } catch (error) {
    console.error('Error toggling mute:', error);
    if (mainWindow) {
      mainWindow.webContents.send('error', 'Error toggling mute');
    }
  }
}

function resetAllVolumes() {
  try {
    Object.keys(audioSources).forEach(sourceId => {
      audioSources[sourceId].volume = 50;
    });
    
    loudness.setVolume(50);
    
    if (mainWindow) {
      mainWindow.webContents.send('reset-volumes', audioSources);
    }
    
    console.log('All volumes reset to 50%');
  } catch (error) {
    console.error('Error resetting volumes:', error);
    if (mainWindow) {
      mainWindow.webContents.send('error', 'Error resetting volumes');
    }
  }
}

// IPC event handlers
ipcMain.on('set-volume', async (event, arg) => {
  try {
    const { source, volume } = arg;
    
    if (!audioSources[source]) {
      throw new Error(`Unknown audio source: ${source}`);
    }
    
    if (volume < 0 || volume > 100) {
      throw new Error(`Invalid volume value: ${volume}`);
    }
    
    audioSources[source].volume = volume;
    
    // For demonstration, adjust master volume
    // In a full implementation, this would adjust the specific application's volume
    await loudness.setVolume(volume);
    
    event.reply('volume-updated', { source, volume });
    console.log(`${audioSources[source].name} volume set to ${volume}%`);
  } catch (error) {
    console.error('Error setting volume via IPC:', error);
    event.reply('error', `Failed to set volume: ${error.message}`);
  }
});

ipcMain.on('get-current-volumes', (event) => {
  event.reply('current-volumes', audioSources);
});

ipcMain.on('toggle-mute', async (event, arg) => {
  try {
    const { source } = arg;
    // For now, toggle master mute (can be extended per source)
    const isMuted = await loudness.getMuted();
    await loudness.setMuted(!isMuted);
    
    event.reply('mute-toggled', { source, muted: !isMuted });
  } catch (error) {
    console.error('Error toggling mute:', error);
    event.reply('error', 'Failed to toggle mute');
  }
});

// App event handlers
app.whenReady().then(() => {
  createWindow();
  
  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('will-quit', () => {
  // Unregister all shortcuts
  globalShortcut.unregisterAll();
  console.log('Global shortcuts unregistered');
});

// Handle app activation on macOS
app.on('activate', () => {
  if (mainWindow === null) {
    createWindow();
  }
});
