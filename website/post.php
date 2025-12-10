<?php
// Read posts metadata
$posts_json = file_get_contents('data/posts.json');
$posts = json_decode($posts_json, true);

// Get slug from URL or default
$slug = isset($_GET['slug']) ? $_GET['slug'] : '404';
$post = null;

foreach ($posts as $p) {
    if ($p['slug'] === $slug) {
        $post = $p;
        break;
    }
}

// 404 handling (simple fallback for now)
if (!$post) {
    $post = $posts[0];
}

// Read content
$content = '';
if (file_exists($post['content_file'])) {
    $content = file_get_contents($post['content_file']);
} else {
    $content = "Content file not found.";
}
?>
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title><?php echo htmlspecialchars($post['title']); ?> - Friendev Blog</title>
    <link rel="stylesheet" href="style.css">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;700&display=swap" rel="stylesheet">
    <!-- Marked.js for Markdown rendering -->
    <script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>
</head>
<body>
    <?php include 'header.php'; ?>

    <article class="blog-post">
        <div class="post-bg-glow"></div>
        <div class="container">
            <div class="post-card animate-on-scroll">
                <div class="post-header">
                    <div class="post-meta">
                        <span class="post-date"><?php echo htmlspecialchars($post['date']); ?></span>
                        <span class="separator">•</span>
                        <span class="post-category"><?php echo htmlspecialchars($post['category']); ?></span>
                    </div>
                    <h1 class="post-title"><?php echo htmlspecialchars($post['title']); ?></h1>
                    <div class="post-author">
                        <div class="author-avatar">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path><circle cx="12" cy="7" r="4"></circle></svg>
                        </div>
                        <div class="author-info">
                            <span class="author-name"><?php echo htmlspecialchars($post['author']); ?></span>
                            <span class="separator">•</span>
                            <span class="read-time"><?php echo htmlspecialchars($post['read_time']); ?></span>
                        </div>
                    </div>
                </div>

                <div class="post-image-container">
                    <?php if (!empty($post['image_url'])): ?>
                        <img src="<?php echo htmlspecialchars($post['image_url']); ?>" alt="<?php echo htmlspecialchars($post['title']); ?>" class="post-featured-image">
                    <?php else: ?>
                        <div class="post-image-placeholder">
                            <div class="placeholder-icon">
                                <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><circle cx="8.5" cy="8.5" r="1.5"></circle><polyline points="21 15 16 10 5 21"></polyline></svg>
                            </div>
                            <span><?php echo htmlspecialchars($post['image_placeholder_text']); ?></span>
                        </div>
                    <?php endif; ?>
                </div>

                <!-- Hidden raw markdown content -->
                <div id="markdown-source" style="display: none;"><?php echo htmlspecialchars($content); ?></div>

                <!-- Rendered content container -->
                <div class="post-content" id="post-content">
                    <!-- Content will be injected here by JS -->
                </div>
            </div>
        </div>
    </article>

    <?php include 'footer.php'; ?>

    <script src="script.js"></script>
    <script>
        // Render Markdown
        document.addEventListener('DOMContentLoaded', () => {
            const rawMarkdown = document.getElementById('markdown-source').innerText;
            const contentDiv = document.getElementById('post-content');
            
            // Configure marked to allow HTML (since we have custom divs in markdown)
            // and highlight code if we had a highlighter, but for now basic parsing
            contentDiv.innerHTML = marked.parse(rawMarkdown);

            // Re-trigger animations for new content if needed
            // (The existing script.js handles .animate-on-scroll, but content is inside .post-card which is already animated)
        });
    </script>
</body>
</html>