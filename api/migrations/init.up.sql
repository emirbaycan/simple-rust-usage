-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS jobs (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    company VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    date VARCHAR(255) NOT NULL,
    description VARCHAR(1000) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    title VARCHAR(255) NOT NULL,
    description VARCHAR(1000) NOT NULL,
    imgs VARCHAR(10000)[] NOT NULL,
    demo VARCHAR(500) NOT NULL,
    git VARCHAR(500) NOT NULL,
    stacks VARCHAR(500)[] NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS testimonials (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    name VARCHAR(255) NOT NULL,
    comment VARCHAR(1000) NOT NULL,
    position VARCHAR(255) NOT NULL,
    company VARCHAR(255) NOT NULL,
    img VARCHAR(500) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS details (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    title VARCHAR(1000) NOT NULL,
    logo VARCHAR(1000) NOT NULL,
    keywords VARCHAR(1000) NOT NULL,
    site_description VARCHAR(1000) NOT NULL,
    description VARCHAR(1000) NOT NULL,
    about VARCHAR(1000) NOT NULL,
    position VARCHAR(255) NOT NULL,
    company VARCHAR(255) NOT NULL,
    img VARCHAR(500) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS images (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    name VARCHAR(1000) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    username VARCHAR(1000) NOT NULL,
    password VARCHAR(1000) NOT NULL,
    email VARCHAR(1000) NOT NULL,
    fullname VARCHAR(1000) NOT NULL,
    role SMALLINT NOT NULL,
    avatar VARCHAR(1000) NOT NULL,
    notes VARCHAR(1000) NOT NULL,
    active SMALLINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

INSERT INTO
    jobs (company, title, date, description)
VALUES
    (
        'Kalenux',
        'Founder',
        '2020 - 2024',
        'Founded Kalenux company for operating an ecommerce business'
    );

INSERT INTO
    projects (title, description, imgs, demo, git, stacks)
VALUES
    (
        'Kalenuxer',
        'A website building framework that provides all the necessary tools for building highly efficient for performance web applications, It contains multi language support, minifiers, obfuscators, SSR, templaters (mail, section, pages..), classification, svg to icons, versionizers, css, js structure and localizations,',
        ARRAY['https://emirbaycan.com.tr/images/kalenuxer/1.webp'],
        'https://github.com/kalinux0/Kalenuxer',
        'https://github.com/kalinux0/Kalenuxer',
        ARRAY['Modifiable Templating Language','HTML/HTML5','CSS/CSS3','Javascript','PHP','Node.js','LAMP','AWS','Git','Figma','MySQL']
    );

INSERT INTO
    details (title, logo, keywords, site_description, description, about, position, company, img)
VALUES
    (
        'Emir Baycan',
        'https://emirbaycan.com.tr/images/logo.webp',
        'Emir Baycan, web developer, software engineer',
        'Results-driven Senior Software Developer base in Turkey with over 5 years of professional experience in web and software development. I have worked on various large-scale projects that prioritized responsive design, performance optimization, and cross-functional collaboration. I pride myself on my ability to translate project requirements into visually appealing and efficient solutions.',
        'I''m a software engineer with <span class=''underline''>over 5 years of experience</span>, specializing in developing systems, interfaces, bots, and technological solutions. I pride myself on my ability to translate project requirements into visually appealing and efficient solutions.',
        'Hey there, I''m Emir Baycan. I hold long variety of skills and currently exploring the world of Mobile App Design and Development. Throughout my career, I''ve been involved in various large-scale projects, prioritizing responsive design, performance optimization, and cross-functional collaboration. I love turning a project''s ''wish list'' into something that not only looks great but also performs well.',
        'Freelancer',
        'Kalenux',
        'https://emirbaycan.com.tr/images/me.webp'
    );

INSERT INTO
    users (username, password, email, fullname, role, avatar, notes, active)
VALUES
    (
        'admin',
        '$2b$12$wC2.kKbuZ9EnAm52GsJfv.U3mxBAxdLqvuP0aTgdnW3UMjm6Nu466',
        'emir-baycan@hotmail.com',
        'Emir Baycan',
        1,
        'https://emirbaycan.com.tr/images/me.webp',
        'Owner',
        1
    );