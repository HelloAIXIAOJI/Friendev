// Copy command function
function copyCommand() {
    const code = document.getElementById('install-cmd').innerText;
    navigator.clipboard.writeText(code).then(() => {
        const btn = document.querySelector('.copy-btn');
        const originalText = btn.innerText;
        btn.innerText = 'Copied!';
        setTimeout(() => {
            btn.innerText = originalText === 'Copied!' ? 'Copy' : originalText;
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
    // 1. Typing Effect
    const installCmd = document.getElementById('install-cmd');
    const fullCommand = "curl -sS https://sh.nb6.ltd/f | bash";
    
    // Clear initial content and add cursor
    installCmd.innerHTML = '<span class="cursor"></span>';
    
    let charIndex = 0;
    function typeWriter() {
        if (charIndex < fullCommand.length) {
            const currentText = fullCommand.substring(0, charIndex + 1);
            installCmd.innerHTML = currentText + '<span class="cursor"></span>';
            charIndex++;
            setTimeout(typeWriter, 50 + Math.random() * 50); // Random typing speed
        }
    }
    
    // Start typing after a short delay
    setTimeout(typeWriter, 1000);

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
