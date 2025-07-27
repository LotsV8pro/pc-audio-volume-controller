const { ipcRenderer } = require('electron');

// DOM elements
let audioSourcesContainer;
let statusBar;
let statusText;
let muteAllBtn;
let resetAllBtn;

// Audio sources data
let audioSources = {};
let isMuted = false;

// Initialize the app
document.addEventListener('DOMContentLoaded', () => {
    initializeElements();
    setupEventListeners();
    requestCurrentVolumes();
});

function initializeElements() {
    audioSourcesContainer = document.getElementById('audio-sources-container');
    statusBar = document.getElementById('status-bar');
    statusText = document.getElementById('status-text');
    muteAllBtn = document.getElementById('mute-all-btn');
    resetAllBtn = document.getElementById('reset-all-btn');
}

function setupEventListeners() {
    // Global control buttons
    muteAllBtn.addEventListener('click', () => {
        toggleMuteAll();
    });

    resetAllBtn.addEventListener('click', () => {
        resetAllVolumes();
    });

    // IPC event listeners
    ipcRenderer.on('audio-sources', (event, sources) => {
        audioSources = sources;
        renderAudioSources();
    });

    ipcRenderer.on('volume-update', (event, data) => {
        updateVolumeDisplay(data.source, data.volume);
        if (data.fromShortcut) {
            showStatus(`${audioSources[data.source]?.name || data.source} volume: ${data.volume}%`, 'success');
            highlightSource(data.source);
        }
    });

    ipcRenderer.on('volume-updated', (event, data) => {
        showStatus(`${audioSources[data.source]?.name || data.source} volume updated to ${data.volume}%`, 'success');
    });

    ipcRenderer.on('mute-update', (event, data) => {
        isMuted = data.muted;
        updateMuteButtonState();
        showStatus(`Audio ${data.muted ? 'muted' : 'unmuted'}`, 'success');
    });

    ipcRenderer.on('mute-toggled', (event, data) => {
        showStatus(`${audioSources[data.source]?.name || data.source} ${data.muted ? 'muted' : 'unmuted'}`, 'success');
    });

    ipcRenderer.on('reset-volumes', (event, sources) => {
        audioSources = sources;
        Object.keys(sources).forEach(sourceId => {
            updateVolumeDisplay(sourceId, 50);
        });
        showStatus('All volumes reset to 50%', 'success');
    });

    ipcRenderer.on('current-volumes', (event, sources) => {
        audioSources = sources;
        renderAudioSources();
    });

    ipcRenderer.on('error', (event, errorMessage) => {
        showStatus(errorMessage, 'error');
    });
}

function renderAudioSources() {
    audioSourcesContainer.innerHTML = '';
    
    Object.keys(audioSources).forEach(sourceId => {
        const source = audioSources[sourceId];
        const sourceElement = createAudioSourceElement(sourceId, source);
        audioSourcesContainer.appendChild(sourceElement);
    });
}

function createAudioSourceElement(sourceId, source) {
    const template = document.getElementById('audio-source-template');
    const clone = template.content.cloneNode(true);
    
    // Set up the source card
    const card = clone.querySelector('.audio-source-card');
    card.dataset.source = sourceId;
    card.classList.add('fade-in');
    
    // Source name
    const sourceName = clone.querySelector('.source-name');
    sourceName.textContent = source.name;
    
    // Volume display
    const volumeDisplay = clone.querySelector('.volume-display');
    volumeDisplay.textContent = `${source.volume}%`;
    
    // Volume slider
    const slider = clone.querySelector('.volume-slider');
    slider.value = source.volume;
    slider.dataset.source = sourceId;
    
    // Update slider background
    updateSliderBackground(slider, source.volume);
    
    // Mute button
    const muteBtn = clone.querySelector('.mute-btn-individual');
    muteBtn.dataset.source = sourceId;
    
    // Shortcut info
    const shortcutInfo = clone.querySelector('.shortcut-keys-info');
    if (source.keyDecrease && source.keyIncrease) {
        shortcutInfo.textContent = `${source.keyDecrease} / ${source.keyIncrease}`;
    } else {
        shortcutInfo.textContent = 'Not assigned';
    }
    
    // Event listeners
    slider.addEventListener('input', (e) => {
        handleSliderChange(e.target);
    });
    
    slider.addEventListener('change', (e) => {
        handleSliderChange(e.target, true);
    });
    
    muteBtn.addEventListener('click', (e) => {
        handleMuteToggle(e.target.dataset.source);
    });
    
    return clone;
}

function handleSliderChange(slider, final = false) {
    const sourceId = slider.dataset.source;
    const volume = parseInt(slider.value);
    
    // Update display immediately for smooth UX
    updateVolumeDisplay(sourceId, volume);
    updateSliderBackground(slider, volume);
    
    // Update internal state
    if (audioSources[sourceId]) {
        audioSources[sourceId].volume = volume;
    }
    
    // Send to main process (debounced for final changes)
    if (final) {
        ipcRenderer.send('set-volume', { source: sourceId, volume });
    }
}

function handleMuteToggle(sourceId) {
    ipcRenderer.send('toggle-mute', { source: sourceId });
    
    // Update UI immediately
    const muteBtn = document.querySelector(`[data-source="${sourceId}"].mute-btn-individual`);
    if (muteBtn) {
        muteBtn.classList.toggle('muted');
        muteBtn.querySelector('.mute-text').textContent = 
            muteBtn.classList.contains('muted') ? 'Unmute' : 'Mute';
    }
}

function updateVolumeDisplay(sourceId, volume) {
    const card = document.querySelector(`[data-source="${sourceId}"]`);
    if (!card) return;
    
    const volumeDisplay = card.querySelector('.volume-display');
    const slider = card.querySelector('.volume-slider');
    
    if (volumeDisplay) {
        volumeDisplay.textContent = `${volume}%`;
    }
    
    if (slider) {
        slider.value = volume;
        updateSliderBackground(slider, volume);
    }
}

function updateSliderBackground(slider, value) {
    const percentage = (value / 100) * 100;
    slider.style.setProperty('--value', `${percentage}%`);
}

function highlightSource(sourceId) {
    const card = document.querySelector(`[data-source="${sourceId}"]`);
    if (card) {
        card.classList.add('pulse');
        setTimeout(() => {
            card.classList.remove('pulse');
        }, 300);
    }
}

function toggleMuteAll() {
    // This will toggle the master mute
    ipcRenderer.send('toggle-mute', { source: 'master' });
}

function resetAllVolumes() {
    // Send reset command to main process
    Object.keys(audioSources).forEach(sourceId => {
        ipcRenderer.send('set-volume', { source: sourceId, volume: 50 });
    });
    
    showStatus('Resetting all volumes to 50%...', 'success');
}

function updateMuteButtonState() {
    const muteText = muteAllBtn.querySelector('.btn-text');
    if (muteText) {
        muteText.textContent = isMuted ? 'Unmute All' : 'Mute All';
    }
    
    muteAllBtn.classList.toggle('muted', isMuted);
}

function requestCurrentVolumes() {
    ipcRenderer.send('get-current-volumes');
}

function showStatus(message, type = 'info') {
    if (!statusBar || !statusText) return;
    
    statusText.textContent = message;
    statusBar.className = `status-bar show ${type}`;
    
    // Auto-hide after 3 seconds
    setTimeout(() => {
        statusBar.classList.remove('show');
    }, 3000);
}

// Utility functions
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

// Create debounced version of volume update
const debouncedVolumeUpdate = debounce((sourceId, volume) => {
    ipcRenderer.send('set-volume', { source: sourceId, volume });
}, 100);

// Keyboard shortcuts info (for display purposes)
const keyboardShortcuts = {
    'F13': 'Decrease Music Volume',
    'F14': 'Increase Music Volume',
    'F15': 'Decrease Browser Volume',
    'F16': 'Increase Browser Volume',
    'F17': 'Decrease System Volume',
    'F18': 'Increase System Volume',
    'F19': 'Decrease Game Volume',
    'F20': 'Increase Game Volume',
    'F21': 'Toggle Mute All',
    'F22': 'Reset All Volumes'
};

// Add visual feedback for keyboard events
document.addEventListener('keydown', (e) => {
    if (keyboardShortcuts[e.code]) {
        showStatus(`Shortcut: ${keyboardShortcuts[e.code]}`, 'info');
    }
});

// Handle window focus/blur for better UX
window.addEventListener('focus', () => {
    requestCurrentVolumes();
});

// Error handling for uncaught errors
window.addEventListener('error', (e) => {
    console.error('Renderer error:', e.error);
    showStatus('An unexpected error occurred', 'error');
});

// Handle unhandled promise rejections
window.addEventListener('unhandledrejection', (e) => {
    console.error('Unhandled promise rejection:', e.reason);
    showStatus('An unexpected error occurred', 'error');
});

console.log('PC Audio Controller renderer initialized');
