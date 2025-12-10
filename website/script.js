// Tab switching
function switchTab(id) {
    // Update buttons
    document.querySelectorAll('.tab-btn').forEach(btn => btn.classList.remove('active'));
    const activeBtn = document.querySelector(`.tab-btn[data-id="${id}"]`);
    if (activeBtn) activeBtn.classList.add('active');

    // Update content
    document.querySelectorAll('.tab-content').forEach(content => {
        content.style.display = 'none';
        content.classList.remove('active');
    });
    
    const activeContent = document.getElementById(`tab-${id}`);
    if (activeContent) {
        activeContent.style.display = 'flex';
        activeContent.classList.add('active');
    }
}

// Copy command function
function copyCommand(elementId, btnElement) {
    const code = document.getElementById(elementId).innerText;
    navigator.clipboard.writeText(code).then(() => {
        const originalText = btnElement.innerText;
        btnElement.innerText = 'Copied!';
        setTimeout(() => {
            btnElement.innerText = originalText === 'Copied!' ? 'Copy' : originalText;
        }, 2000);
    });
}

// Feature card hover effect
document.querySelectorAll('.feature-card').forEach(card => {
    card.addEventListener('mousemove', e => {
        const rect = card.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        card.style.setProperty('--mouse-x', `${x}px`);
        card.style.setProperty('--mouse-y', `${y}px`);
    });
});

// Scroll animations and other effects
document.addEventListener('DOMContentLoaded', () => {
    // Fetch install commands
    fetch('install.json')
        .then(response => response.json())
        .then(config => {
            // 1. Render Hero Section (Tabs + Content)
            const heroBox = document.getElementById('hero-install-box');
            if (heroBox && config.length > 0) {
                heroBox.innerHTML = ''; // Clear placeholder

                // Create Tabs Container
                const tabsContainer = document.createElement('div');
                tabsContainer.className = 'install-tabs';
                
                config.forEach((item, index) => {
                    // Create Tab Button
                    const btn = document.createElement('button');
                    btn.className = `tab-btn ${index === 0 ? 'active' : ''}`;
                    btn.innerText = item.label;
                    btn.setAttribute('data-id', item.id);
                    btn.onclick = () => switchTab(item.id);
                    tabsContainer.appendChild(btn);
                });

                heroBox.appendChild(tabsContainer);
                
                // Append content divs
                config.forEach((item, index) => {
                    const contentDiv = document.createElement('div');
                    contentDiv.className = `tab-content ${index === 0 ? 'active' : ''}`;
                    contentDiv.id = `tab-${item.id}`;
                    contentDiv.style.display = index === 0 ? 'flex' : 'none';
                    
                    contentDiv.innerHTML = `
                        <span class="prompt">${item.prompt}</span>
                        <code id="install-cmd-${item.id}">${item.command}</code>
                        <button class="copy-btn" onclick="copyCommand('install-cmd-${item.id}', this)">Copy</button>
                    `;
                    heroBox.appendChild(contentDiv);
                });

                // Typing Effect for the first item
                const firstItem = config[0];
                const installCmd = document.getElementById(`install-cmd-${firstItem.id}`);
                if (installCmd) {
                    const fullCommand = firstItem.command;
                    installCmd.innerHTML = '<span class="cursor"></span>';
                    
                    let charIndex = 0;
                    function typeWriter() {
                        if (charIndex < fullCommand.length) {
                            const currentText = fullCommand.substring(0, charIndex + 1);
                            installCmd.innerHTML = currentText + '<span class="cursor"></span>';
                            charIndex++;
                            setTimeout(typeWriter, 50 + Math.random() * 50);
                        }
                    }
                    setTimeout(typeWriter, 1000);
                }
            }

            // 2. Render Quick Install Section
            const quickInstallContainer = document.getElementById('quick-install-container');
            if (quickInstallContainer && config.length > 0) {
                quickInstallContainer.innerHTML = '';
                
                config.forEach(item => {
                    const div = document.createElement('div');
                    div.className = 'os-install';
                    div.innerHTML = `
                        <h4>${item.label}</h4>
                        <pre><code id="quick-install-${item.id}">${item.command}</code></pre>
                    `;
                    quickInstallContainer.appendChild(div);
                });
            }
        })
        .catch(error => {
            console.error('Error loading install config:', error);
            const heroBox = document.getElementById('hero-install-box');
            if (heroBox) heroBox.innerHTML = '<div class="loading-placeholder">Error loading installation options.</div>';
        });

    // 2. Number Counter Animation
    const statsObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                const statNumber = entry.target.querySelector('.number');
                if (statNumber && !statNumber.classList.contains('counted')) {
                    animateCounter(statNumber);
                    statNumber.classList.add('counted');
                }
            }
        });
    }, { threshold: 0.5 });

    function animateCounter(el) {
        const targetText = el.getAttribute('data-target') || el.innerText;
        // Extract number and suffix (e.g., "100%" -> 100, "%")
        const match = targetText.match(/(\d+)(.*)/);
        if (!match) return;
        
        const targetValue = parseInt(match[1], 10);
        const suffix = match[2];
        const duration = 2000; // 2 seconds
        const startTime = performance.now();
        
        function update(currentTime) {
            const elapsed = currentTime - startTime;
            const progress = Math.min(elapsed / duration, 1);
            
            // Ease out quart
            const ease = 1 - Math.pow(1 - progress, 4);
            
            const currentValue = Math.floor(targetValue * ease);
            el.innerText = currentValue + suffix;
            
            if (progress < 1) {
                requestAnimationFrame(update);
            } else {
                el.innerText = targetText; // Ensure exact final value
            }
        }
        
        requestAnimationFrame(update);
    }

    document.querySelectorAll('.stat').forEach(stat => {
        statsObserver.observe(stat);
    });

    // Existing Intersection Observer for fade-in animations
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('visible');
                // Optional: stop observing once visible
                // observer.unobserve(entry.target);
            }
        });
    }, observerOptions);

    const animatedElements = document.querySelectorAll('.animate-on-scroll, .feature-card, .step, .stat');
    animatedElements.forEach(el => {
        el.classList.add('animate-hidden');
        observer.observe(el);
    });
});