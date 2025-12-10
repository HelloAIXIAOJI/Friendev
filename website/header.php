<?php
$current_page = basename($_SERVER['PHP_SELF']);
$is_home = ($current_page == 'index.php' || $current_page == '');
?>
<header>
    <div class="container">
        <div class="header-top">
            <div class="logo"><a href="index.php" style="text-decoration:none; color:inherit;">Friendev</a></div>
            <button class="menu-toggle" aria-label="Toggle navigation">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="3" y1="12" x2="21" y2="12"></line><line x1="3" y1="6" x2="21" y2="6"></line><line x1="3" y1="18" x2="21" y2="18"></line></svg>
            </button>
        </div>
        <nav id="main-nav">
            <a href="<?php echo $is_home ? '#features' : 'index.php#features'; ?>">Features</a>
            <a href="<?php echo $is_home ? '#install' : 'index.php#install'; ?>">Install</a>
            <a href="postlist.php" target="_blank">Blog</a>
            <a href="https://github.com/helloaixiaoji/friendev" target="_blank">Github</a>
            <a href="https://github.com/helloaixiaoji/friendev/discussions" target="_blank">Discussions</a>
        </nav>
    </div>
    <script>
        document.querySelector('.menu-toggle').addEventListener('click', function() {
            const nav = document.getElementById('main-nav');
            const isExpanded = nav.classList.contains('expanded');
            if (isExpanded) {
                nav.classList.remove('expanded');
                this.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="3" y1="12" x2="21" y2="12"></line><line x1="3" y1="6" x2="21" y2="6"></line><line x1="3" y1="18" x2="21" y2="18"></line></svg>';
            } else {
                nav.classList.add('expanded');
                this.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>';
            }
        });
    </script>
</header>
