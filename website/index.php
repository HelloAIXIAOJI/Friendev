<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Friendev - AI-Powered Development Assistant</title>
    <link rel="stylesheet" href="style.css">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;700&display=swap" rel="stylesheet">
</head>
<body>
    <?php include 'header.php'; ?>
    <main>
    <section class="hero">
        <div class="container">
            <h1>AI-Powered<br>Development Assistant</h1>
            <p class="subtitle">An interactive REPL interface for AI-powered coding assistance. Coding has never been this seamless.</p>
            
            <div class="install-box" id="hero-install-box">
                <!-- Dynamic content loaded from install.json -->
                <div class="loading-placeholder">
                    <div class="spinner"></div>
                    <span>Loading installation options...</span>
                </div>
            </div>
        </div>
    </section>

    <section id="features" class="features">
        <div class="container">
            <h2 class="animate-on-scroll">Workflow Evolved.</h2>
            <div class="feature-grid">
                <!-- Feature 1: REPL (Full Width) -->
                <div class="feature-card full-width">
                    <div class="card-content">
                        <div class="card-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"></path></svg>
                        </div>
                        <h3>Interactive REPL</h3>
                        <p>A powerful Read-Eval-Print Loop designed for seamless interaction with AI models directly from your terminal. Coding, redefined.</p>
                    </div>
                    <div class="card-visual terminal-preview">
                        <div class="terminal-body">
                            <div class="line"><span class="prompt">></span> ask "how to center a div?"</div>
                            <div class="line output">To center a div in CSS,...</div>
                        </div>
                    </div>
                </div>

                <!-- Feature 2: Approvals -->
                <div class="feature-card">
                    <div class="card-icon">
                        <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path></svg>
                    </div>
                    <h3>Smart Approvals</h3>
                    <p>Security meets speed. Use <code>--shorekeeper</code> mode for AI-reviewed prompt approvals.</p>
                </div>

                <!-- Feature 3: Performance -->
                <div class="feature-card">
                    <div class="card-icon">
                        <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z"></path><path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z"></path><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0"></path><path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"></path></svg>
                    </div>
                    <h3>Native Performance</h3>
                    <p>Built with Rust. Blazing fast. Runs on Windows, Linux, macOS, Android, and FreeBSD.</p>
                </div>

                <!-- Feature 4: Toolset (Full Width) -->
                <div class="feature-card full-width reverse">
                    <div class="card-content">
                        <div class="card-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"></path></svg>
                        </div>
                        <h3>Integrated Toolset</h3>
                        <p>File editing, network access, and system command execution. All the tools you need, right where you need them.</p>
                    </div>
                    <div class="card-visual tool-grid">
                        <div class="tool-badge">file_*</div>
                        <div class="tool-badge">network_*</div>
                        <div class="tool-badge">run_command</div>
                        <div class="tool-badge">todo_*</div>
                        <div class="tool-badge">Sub-Agent</div>
                        <div class="tool-badge">MCP</div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section class="flow-section">
        <div class="flow-bg-glow"></div>
        <div class="container">
            <div class="flow-content">
                <h2 class="animate-on-scroll">Stay in the Flow.</h2>
                <p class="animate-on-scroll">Stop tab-switching. Friendev brings collective intelligence directly to your terminal, keeping your creative momentum unbroken.</p>
                
                <div class="flow-dashboard animate-on-scroll">
                    <div class="dashboard-header">
                        <div class="win-controls">
                            <span></span><span></span><span></span>
                        </div>
                    </div>
                    <div class="dashboard-grid">
                        <!-- Stat 1 -->
                        <div class="dash-card">
                            <div class="dash-icon">
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path><polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline><line x1="12" y1="22.08" x2="12" y2="12"></line></svg>
                            </div>
                            <div class="dash-info">
                                <span class="dash-label">Local Context</span>
                                <span class="dash-value">100%</span>
                            </div>
                        </div>

                        <!-- Stat 2 -->
                        <div class="dash-card">
                            <div class="dash-icon">
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon></svg>
                            </div>
                            <div class="dash-info">
                                <span class="dash-label">Context Switch</span>
                                <span class="dash-value">0ms</span>
                            </div>
                        </div>

                        <!-- Stat 3 -->
                        <div class="dash-card">
                            <div class="dash-icon">
                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="2" y1="12" x2="22" y2="12"></line><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path></svg>
                            </div>
                            <div class="dash-info">
                                <span class="dash-label">Availability</span>
                                <span class="dash-value">24/7</span>
                            </div>
                        </div>
                    </div>
                </div>
                
                <p class="disclaimer">* Performance depends on hardware and API provider.</p>
            </div>
        </div>
    </section>

    <section class="architecture">
        <div class="container">
            <h2 class="animate-on-scroll">Under the Hood</h2>
            <div class="arch-flow">
                <!-- Connecting Line -->
                <div class="flow-line">
                    <div class="flow-pulse"></div>
                </div>

                <!-- Step 1 -->
                <div class="arch-step animate-on-scroll">
                    <div class="step-number">01</div>
                    <div class="step-card">
                        <div class="step-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9.5 2A2.5 2.5 0 0 1 12 4.5v15a2.5 2.5 0 0 1-4.96.44 2.5 2.5 0 0 1-2.96-3.08 3 3 0 0 1-.34-5.58 2.5 2.5 0 0 1 1.32-4.24 2.5 2.5 0 0 1 1.98-3A2.5 2.5 0 0 1 9.5 2Z"></path><path d="M14.5 2A2.5 2.5 0 0 0 12 4.5v15a2.5 2.5 0 0 0 4.96.44 2.5 2.5 0 0 0 2.96-3.08 3 3 0 0 0 .34-5.58 2.5 2.5 0 0 0-1.32-4.24 2.5 2.5 0 0 0-1.98-3A2.5 2.5 0 0 0 14.5 2Z"></path></svg>
                        </div>
                        <div class="step-content">
                            <h4>LLM Core</h4>
                            <p>Connects to advanced AI models for context-aware reasoning.</p>
                        </div>
                    </div>
                </div>

                <!-- Step 2 -->
                <div class="arch-step animate-on-scroll">
                    <div class="step-number">02</div>
                    <div class="step-card">
                        <div class="step-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>
                        </div>
                        <div class="step-content">
                            <h4>Tool Engine</h4>
                            <p>Safely executes shell commands and file operations.</p>
                        </div>
                    </div>
                </div>

                <!-- Step 3 -->
                <div class="arch-step animate-on-scroll">
                    <div class="step-number">03</div>
                    <div class="step-card">
                        <div class="step-icon">
                            <svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 10 0v4"></path></svg>
                        </div>
                        <div class="step-content">
                            <h4>Safety Layer</h4>
                            <p>Shorekeeper mode validates risky actions before execution.</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section id="community" class="community-section">
        <div class="container">
            <div class="community-content">
                <h2 class="animate-on-scroll">Built in the Open.</h2>
                <p class="animate-on-scroll">Transparency builds trust. Friendev is fully open source and community-driven. Join us in shaping the future of AI-assisted development.</p>
                
                <div class="repo-card animate-on-scroll">
                    <div class="repo-header">
                        <div class="repo-title">
                            <svg height="24" viewBox="0 0 16 16" version="1.1" width="24" fill="currentColor"><path d="M2 2.5A2.5 2.5 0 0 1 4.5 0h8.75a.75.75 0 0 1 .75.75v12.5a.75.75 0 0 1-.75.75h-2.5a.75.75 0 0 1 0-1.5h1.75v-2h-8a1 1 0 0 0-.714 1.7.75.75 0 1 1-1.072 1.05A2.495 2.495 0 0 1 2 11.5Zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 0 1 1-1h8ZM5 12.25a.25.25 0 0 1 .25-.25h3.5a.25.25 0 0 1 .25.25v3.25a.25.25 0 0 1-.4.2l-1.45-1.087a.249.249 0 0 0-.3 0L5.4 15.7a.25.25 0 0 1-.4-.2Z"></path></svg>
                            <span>helloaixiaoji / friendev</span>
                        </div>
                        <div class="repo-badge">Public</div>
                    </div>
                    <div class="repo-actions">
                        <a href="https://github.com/helloaixiaoji/friendev" target="_blank" class="btn-github">
                            Star on GitHub
                        </a>
                        <a href="#" class="btn-discord">
                            Join Telegram
                        </a>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <section id="install" class="installation">
        <div class="container">
            <h2 class="animate-on-scroll">Get Started</h2>
            <div class="install-options">
                <div class="option animate-on-scroll">
                    <div class="option-header">
                        <h3>Quick Install</h3>
                        <p>The fastest way to get Friendev running on your system.</p>
                    </div>
                    <div class="option-content">
                        <div id="quick-install-container">
                            <!-- Dynamic content loaded from install.json -->
                        </div>
                    </div>
                </div>

                <div class="option animate-on-scroll">
                    <div class="option-header">
                        <h3>Build from Source</h3>
                        <p>Requires Rust 1.70+ and Cargo.</p>
                    </div>
                    <div class="option-content">
                        <pre><code><span class="cmd-prefix">$</span>git clone https://github.com/helloaixiaoji/friendev.git</code><button class="copy-btn-code" onclick="copyText(this)">COPY</button></pre>
                        <div style="height: 1rem;"></div>
                        <pre><code><span class="cmd-prefix">$</span>cargo run --release</code><button class="copy-btn-code" onclick="copyText(this)">COPY</button></pre>
                    </div>
                </div>

                <div class="option animate-on-scroll">
                    <div class="option-header">
                        <h3>Download Release</h3>
                        <p>Get the latest binary for your platform from GitHub Releases.</p>
                    </div>
                    <div class="option-content">
                        <a href="https://github.com/helloaixiaoji/friendev/releases/latest" target="_blank" class="btn-download">
                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>
                            Go to Releases
                        </a>
                    </div>
                </div>
            </div>
        </div>
    </section>
</main>
    <?php include 'footer.php'; ?>

    <script src="script.js"></script>
</body>
</html>
