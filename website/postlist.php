<?php
// Read posts metadata
$posts_json = file_get_contents('data/posts.json');
$posts = json_decode($posts_json, true);

// Sort posts by date (newest first)
usort($posts, function($a, $b) {
    // Simple date comparison - assuming format like "2025 12 11"
    $dateA = strtotime($a['date']);
    $dateB = strtotime($b['date']);
    return $dateB - $dateA;
});

// Get category filter from URL
$category = isset($_GET['category']) ? $_GET['category'] : null;

// Filter posts by category if specified
if ($category) {
    $filtered_posts = array_filter($posts, function($post) use ($category) {
        return strtolower($post['category']) === strtolower($category);
    });
} else {
    $filtered_posts = $posts;
}

// Get unique categories for filter
$categories = array_unique(array_column($posts, 'category'));
sort($categories);
?>
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Blog Posts - Friendev</title>
    <link rel="stylesheet" href="style.css">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;700&display=swap" rel="stylesheet">
</head>
<body>
    <?php include 'header.php'; ?>

    <main class="blog-list">
        <div class="post-bg-glow"></div>
        <div class="container">
            <div class="page-header">
                <h1>Blog Posts</h1>
                <p>Latest articles about Friendev and AI programming</p>
            </div>

            <!-- Category Filter -->
            <div class="category-filter">
                <button class="filter-btn <?php echo !$category ? 'active' : ''; ?>" onclick="window.location.href='postlist.php'">All Posts</button>
                <?php foreach ($categories as $cat): ?>
                    <?php if ($cat && $cat !== '404'): ?>
                        <button class="filter-btn <?php echo strtolower($cat) === strtolower($category) ? 'active' : ''; ?>" 
                                onclick="window.location.href='postlist.php?category=<?php echo urlencode($cat); ?>'">
                            <?php echo htmlspecialchars($cat); ?>
                        </button>
                    <?php endif; ?>
                <?php endforeach; ?>
            </div>

            <!-- Posts Grid -->
            <div class="posts-grid">
                <?php foreach ($filtered_posts as $post): ?>
                    <?php if ($post['slug'] !== '404'): ?>
                        <article class="post-card animate-on-scroll">
                            <div class="post-image-container">
                                <?php if (!empty($post['image_url'])): ?>
                                    <img src="<?php echo htmlspecialchars($post['image_url']); ?>" 
                                         alt="<?php echo htmlspecialchars($post['title']); ?>" 
                                         class="post-featured-image">
                                <?php else: ?>
                                    <div class="post-image-placeholder">
                                        <div class="placeholder-icon">
                                            <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round">
                                                <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
                                                <circle cx="8.5" cy="8.5" r="1.5"></circle>
                                                <polyline points="21 15 16 10 5 21"></polyline>
                                            </svg>
                                        </div>
                                        <span><?php echo htmlspecialchars($post['image_placeholder_text']); ?></span>
                                    </div>
                                <?php endif; ?>
                            </div>

                            <div class="post-content">
                                <div class="post-meta">
                                    <span class="post-date"><?php echo htmlspecialchars($post['date']); ?></span>
                                    <span class="separator">•</span>
                                    <span class="post-category"><?php echo htmlspecialchars($post['category']); ?></span>
                                </div>
                                
                                <h2 class="post-title">
                                    <a href="post.php?slug=<?php echo urlencode($post['slug']); ?>">
                                        <?php echo htmlspecialchars($post['title']); ?>
                                    </a>
                                </h2>

                                <div class="post-author">
                                    <div class="author-avatar">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                                            <circle cx="12" cy="7" r="4"></circle>
                                        </svg>
                                    </div>
                                    <div class="author-info">
                                        <span class="author-name"><?php echo htmlspecialchars($post['author']); ?></span>
                                        <span class="separator">•</span>
                                        <span class="read-time"><?php echo htmlspecialchars($post['read_time']); ?></span>
                                    </div>
                                </div>

                                <div class="post-excerpt">
                                    <?php 
                                    // Read content and generate excerpt
                                    $content = '';
                                    if (file_exists($post['content_file'])) {
                                        $content = file_get_contents($post['content_file']);
                                    }
                                    
                                    // Remove markdown and get first 150 characters
                                    $plain_text = strip_tags(preg_replace('/[#`*_\[\]()]/', '', $content));
                                    $excerpt = substr($plain_text, 0, 150);
                                    if (strlen($plain_text) > 150) {
                                        $excerpt .= '...';
                                    }
                                    echo htmlspecialchars($excerpt);
                                    ?>
                                </div>

                                <a href="post.php?slug=<?php echo urlencode($post['slug']); ?>" class="read-more">
                                    Read More <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14M12 5l7 7-7 7"></path></svg>
                                </a>
                            </div>
                        </article>
                    <?php endif; ?>
                <?php endforeach; ?>

                <?php if (empty($filtered_posts) || (count($filtered_posts) === 1 && $filtered_posts[0]['slug'] === '404')): ?>
                    <div class="no-posts">
                        <h3>No posts found</h3>
                        <p>Check back later for new articles!</p>
                    </div>
                <?php endif; ?>
            </div>
        </div>
    </main>

    <?php include 'footer.php'; ?>

    <script src="script.js"></script>
</body>
</html>