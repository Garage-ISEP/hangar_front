use crate::
{
    components::{footer::Footer, nav::Nav},
    contexts::user_context::UserProvider,
    router::{switch, AppRoute},
};
use i18nrs::yew::{I18nProvider, I18nProviderConfig};
use std::collections::HashMap;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html 
{
    let translations = HashMap::from([
        (
            "en",
            r#"{
                "common": {
                    "loading": "Loading...",
                    "error": "An error occurred.",
                    "owner": "Owner",
                    "source_url": "Source URL",
                    "deployed_image": "Deployed Image",
                    "status": "Status",
                    "created_on": "Created on: {date}",
                    "back_to_home": "Back to home",
                    "status_running": "Running",
                    "status_exited": "Exited",
                    "status_stopped": "Stopped",
                    "status_dead": "Dead",
                    "status_restarting": "Restarting",
                    "status_created": "Created",
                    "status_paused": "Paused",
                    "status_unknown": "Unknown"
                },
                "home": {
                    "title": "Welcome to Hangar",
                    "login_button": "Login with Moodle",
                    "description": "Easily deploy and manage your applications."
                },
                "nav": {
                    "home": "Home",
                    "admin": "Admin",
                    "logout": "Logout"
                },
                "footer": {
                    "about": "About",
                    "terms": "Terms of Use",
                    "privacy": "Privacy Policy",
                    "contact": "Contact"
                },
                "about": {
                    "title": "About Hangar",
                    "hero_subtitle": "Garage Isep's deployment platform",
                    "what_is_title": "What is Hangar?",
                    "what_is_p1": "Hangar is an application deployment platform developed by Garage Isep. It allows ISEP students to easily deploy and manage their web applications without worrying about technical infrastructure.",
                    "what_is_p2": "Whether you want to host your Web APP project, your final year project, or something else, Hangar provides all the necessary tools in just a few clicks.",
                    
                    "features_title": "Main Features",
                    "feature_deploy_title": "Simplified Deployment",
                    "feature_deploy_desc": "Connect your GitHub repository or use a Docker image. Hangar takes care of the rest: build, security scan, and automatic deployment.",
                    "feature_database_title": "MariaDB Databases",
                    "feature_database_desc": "Create and manage your databases in one click.",
                    "feature_monitoring_title": "Real-time Monitoring",
                    "feature_monitoring_desc": "Monitor the state of your applications, check logs, and view resource usage (CPU, RAM) through intuitive dashboards.",
                    "feature_collab_title": "Collaboration",
                    "feature_collab_desc": "Add participants to your projects to work as a team.",
                    "feature_security_title": "Security",
                    "feature_security_desc": "Automatic vulnerability scanning, secret encryption, container isolation, and automatic HTTPS with Let's Encrypt.",
                    "feature_updates_title": "Zero-downtime Updates",
                    "feature_updates_desc": "Deploy new versions without service interruption thanks to the blue-green deployment system.",
                    
                    "tech_title": "Technologies Used",
                    "tech_intro": "Hangar is fully built in Rust to ensure performance, security, and reliability:",
                    "tech_frontend": "Frontend: Yew (WebAssembly)",
                    "tech_backend": "Backend: Axum (REST API)",
                    "tech_container": "Containerization: Docker + Traefik",
                    "tech_db": "Databases: PostgreSQL + MariaDB",
                    
                    "team_title": "The Team",
                    "team_p1": "Hangar is developed by Simon TULOUP and maintained by Garage Isep's IT team.",
                    "team_p2": "This project is new. If you have any feedback, feel free to contact us!",
                    
                    "mission_title": "Our Mission",
                    "mission_p1": "Make application deployment accessible to all students, regardless of their technical level.",
                    "mission_p2": "Provide a robust, modern, and secure infrastructure to help you."
                },
                "terms": {
                    "title": "Terms of Use",
                    "last_updated": "Last updated: October 2025",
                    "intro": "By using Hangar, you agree to the following terms. Please read them carefully.",
                    
                    "h2_1": "1. Acceptance of Terms",
                    "p1_1": "By accessing Hangar and deploying applications, you agree to be bound by these terms of use. If you do not accept them, you must not use the platform.",
                    "p1_2": "These terms may be modified at any time. Changes take effect immediately upon publication on this page.",
                    
                    "h2_2": "2. Eligibility",
                    "p2_1": "Hangar is exclusively reserved for Isep students.",
                    "p2_2": "You must have valid Isep credentials to access the platform.",
                    
                    "h2_3": "3. User Responsibility",
                    "p3_1": "You are solely responsible for the content you deploy on Hangar. You agree to:",
                    "p3_list_1": "Not host illegal, defamatory, hateful content, or content infringing the rights of others",
                    "p3_list_2": "Not deploy malicious code, viruses, or any harmful elements",
                    "p3_list_3": "Not use the platform for commercial activities without authorization",
                    "p3_list_4": "Respect intellectual property rights (software licenses, copyrights)",
                    "p3_list_5": "Not compromise the security or stability of the platform",
                    "p3_2": "Garage Isep reserves the right to remove without notice any project that violates these rules and to suspend or ban the concerned user account.",
                    
                    "h2_4": "4. Resource Limitations",
                    "p4_1": "To ensure fair service quality for everyone, each user is subject to the following limits:",
                    "p4_list_1": "Maximum 1 project per user",
                    "p4_list_2": "Maximum 1 database per user",
                    "p4_list_3": "512 MB RAM per container",
                    "p4_list_4": "50% of one CPU core per container",
                    "p4_list_5": "Reasonable storage for persistent volumes",
                    "p4_2": "Any abuse or excessive resource use (cryptocurrency mining, brute force attacks, etc.) may result in immediate suspension of the project without notice.",
                    
                    "h2_5": "5. Service Availability",
                    "p5_1": "Garage Isep strives to keep Hangar available 24/7 but cannot guarantee 100% uptime.",
                    "p5_2": "Scheduled maintenance may cause temporary interruptions. We strive to announce them in advance.",
                    "p5_3": "Garage Isep cannot be held responsible for data loss, service interruptions, or any damage resulting from the use of Hangar.",
                    
                    "h2_6": "6. Data Backup",
                    "p6_1": "Although we perform regular infrastructure backups, you are responsible for backing up your own data (code, database, files).",
                    "p6_2": "Garage Isep cannot be held responsible for any data loss resulting from error, technical failure, or project deletion.",
                    
                    "h2_7": "7. Privacy and Security",
                    "p7_1": "Your login credentials are managed via Isep’s CAS system. We do not store your passwords.",
                    "p7_2": "Environment variables and database passwords are encrypted in the database (AES-256-GCM).",
                    "p7_3": "Access logs are retained for security and debugging purposes.",
                    "p7_4": "You are responsible for the security of your deployed applications. Hangar provides tools (vulnerability scans, isolation) but cannot guarantee the security of your code.",
                    
                    "h2_8": "8. Intellectual Property",
                    "p8_1": "You retain all rights to the code and content you deploy on Hangar.",
                    "p8_2": "By using Hangar, you grant Garage Isep a non-exclusive license to host, run, and distribute your content as part of the service.",
                    "p8_3": "Hangar’s source code is open-source and available under the appropriate license.",
                    
                    "h2_9": "9. Termination",
                    "p9_1": "You can delete your project at any time from the user interface.",
                    "p9_2": "Garage Isep reserves the right to suspend or delete any project in case of a breach of these terms.",
                    "p9_3": "Upon graduation or leaving Isep, your account may be deactivated after a grace period.",
                    
                    "h2_10": "10. Contact and Support",
                    "p10_1": "For any questions about these terms of use, contact us at dsi@garageisep.com",
                    "p10_2": "Technical support is provided as much as possible, without any guarantee of response time.",
                    
                    "acceptance": "By using Hangar, you confirm that you have read, understood, and accepted these terms of use."
                },
                "privacy": {
                    "title": "Privacy Policy – Hangar",
                    "last_updated": "Last updated: December 2025",
                    "intro": "This privacy policy informs Hangar users how their data is collected, used, and protected in accordance with the GDPR and French Data Protection Law.",

                    "h2_1": "1. Data Controller",
                    "p1": "Garage Isep\nFrench non-profit association – R.N.A: W751249825\nAddress: 28 Rue Notre Dame des Champs, 75006 Paris, France\nEmail: dsi@garageisep.com",

                    "h2_2": "2. Personal Data Collected",
                    "p2_intro": "The following data may be collected:",
                    "p2_id": "• Identification data: first name, ISEP email, ISEP user ID",
                    "p2_tech": "• Technical data: IP address, connection logs, project information, application logs",
                    "p2_service": "• Service-related data: environment variables, encrypted database credentials, resources used",
                    "p2_note": "No user passwords are stored by Hangar (CAS authentication).",

                    "h2_3": "3. Purposes of Processing",
                    "p3_1": "Provide and manage access to Hangar",
                    "p3_2": "Enable deployment and hosting of applications",
                    "p3_3": "Ensure security and prevent abuse",
                    "p3_4": "Monitor performance and stability",
                    "p3_5": "Technical support and maintenance",
                    "p3_6": "Compliance with legal obligations",

                    "h2_4": "4. Legal Basis",
                    "p4_1": "Service execution (Art. 6.1.b GDPR)",
                    "p4_2": "Legitimate interest (Art. 6.1.f GDPR)",
                    "p4_3": "Legal obligations (Art. 6.1.c GDPR)",

                    "h2_5": "5. Data Retention",
                    "p5": "Account data: duration of service use\nLogs: max 12 months\nProject data: deleted when project is deleted",

                    "h2_6": "6. Data Recipients",
                    "p6": "Authorized DSI members and necessary technical administrators. Data is not sold or shared.",

                    "h2_7": "7. Data Hosting",
                    "p7": "Hosting in France / EU. Technical and organizational measures ensure security and confidentiality.",

                    "h2_8": "8. Data Security",
                    "p8": "AES-256-GCM encryption, restricted access, container isolation, HTTPS, regular backups. No system is completely risk-free.",

                    "h2_9": "9. User Rights",
                    "p9": "Right to access, rectify, erase, restrict, object, and portability. Contact: dsi@garageisep.com, response within 30 days.",

                    "h2_10": "10. Account Deletion",
                    "p10": "Deletion possible via interface or email, also deletes associated projects and data.",

                    "h2_11": "11. Cookies and Trackers",
                    "p11": "Only strictly necessary cookies are used. No advertising or commercial tracking cookies.",

                    "h2_12": "12. Policy Updates",
                    "p12": "Garage Isep may update this policy at any time. Users are informed via the platform.",

                    "h2_13": "13. Complaints to CNIL",
                    "p13": "French Data Protection Authority (CNIL) – www.cnil.fr",

                    "contact": "Contact: dsi@garageisep.com"
                    },
                "contact": {
                    "title": "Contact",
                    "p1": "If you have any questions, encounter a problem, or have suggestions for improvement, please do not hesitate to contact the Garage Isep team.",
                    "p2_prefix": "You can reach us by email at: "
                },
                "auth": {
                    "logging_in": "Connecting, please wait...",
                    "login_failed": "Authentication failed. Please try again.",
                    "ticket_missing": "Authentication ticket is missing. Please try logging in again."
                },
                "dashboard": {
                    "welcome": "Welcome, {name}!",
                    "description": "Your application deployment center.",
                    "create_project_button": "New project or database",
                    "owned_projects_title": "My Projects & Databases",
                    "participating_projects_title": "My participations",
                    "empty_state_owned": "You don't own any projects or databases yet.",
                    "empty_state_participating": "You are not participating in any projects."
                },
                "create_project": {
                    "title": "Create a new project",
                    "documentation": "Please refer to the ",
                    "github_tab": "Deploy from GitHub",
                    "direct_tab": "Advanced: Deploy from Image",
                    "description_github": "The easiest way. Your code will be automatically built and deployed from a public GitHub repository.",
                    "description_direct": "For advanced users. Deploy a service from a **public** Docker image. Note: Private images from `ghcr.io` are not supported in this mode.",
                    "name_label": "Project name",
                    "name_placeholder": "my-awesome-app",
                    "name_help": "Will be used for the URL (e.g., my-awesome-app.garageisep.com). Only letters, numbers, and hyphens.",
                    "github_repo_url_label": "GitHub repository URL",
                    "github_repo_url_placeholder": "https://github.com/user/my-project",
                    "image_label": "Docker image URL",
                    "image_placeholder": "my-registry/my-image:1.0",
                    "participants_label": "Participants",
                    "participants_placeholder": "situ62394, john.doe",
                    "participants_help": "MOODLE logins, separated by commas. They will have access to the project.",
                    "submit_button": "Deploy project",
                    "submit_button_loading": "Deploying...",
                    "link_github_prompt": "To deploy from GitHub, you must first install the Hangar GitHub App on your account or organization.",
                    "link_github_button": "Install GitHub App",
                    "github_connected_success": "GitHub App connected successfully! You can now retry deploying your project.",
                    "volume_path_label": "Persistent Volume Path (Optional)",
                    "volume_path_help": "Path inside the container to persist.",
                    "env_vars_label": "Environment Variables (Optional)",
                    "env_vars_help": "One variable per line, in KEY=VALUE format.",
                    "database_tab": "Database Only",
                    "description_database": "Create a standalone MariaDB database, without linking it to a project.",
                    "create_db_checkbox": "Also create and link a new database",
                    "github_branch_help": "Leave empty to use the default branch.",
                    "github_branch_label": "GitHub branch (Optional)",
                    "github_root_dir_label": "Root Directory (Optional)",
                    "github_root_dir_help": "Relative path to the folder containing index.php. Example: /src/public. Leave empty if index.php is at the project root."
                },
                "project_dashboard": {
                    "title": "Project dashboard",
                    "visit_app_button": "Visit App",
                    "card_title_info": "Project info",
                    "card_title_controls": "Controls",
                    "card_title_logs": "Logs",
                    "card_title_metrics": "Metrics (in %)",
                    "card_title_danger": "Danger zone",
                    "card_title_update_image": "Update image",
                    "card_title_rebuild": "Rebuild from GitHub",
                    "card_title_env_vars": "Manage Environment Variables",
                    "logs_placeholder": "Click 'Fetch logs' to display container logs",
                    "logs_empty": "No log output. The container might not be logging to stdout/stderr, or it's just quiet.",
                    "logs_error": "Error fetching logs: {error}",
                    "delete_button": "Delete project",
                    "confirm_delete": "Are you sure you want to permanently delete the project '{name}'? This action is irreversible.",
                    "confirm_delete_db_warning": "The linked database will also be permanently deleted.",
                    "access_error_title": "Access error",
                    "load_error_message": "Could not load project: {error}",
                    "start_button": "Start",
                    "stop_button": "Stop",
                    "restart_button": "Restart",
                    "start_success": "Project started successfully!",
                    "stop_success": "Project stopped successfully!",
                    "restart_success": "Project restarted successfully!",
                    "fetch_logs_button": "Fetch logs",
                    "fetch_logs_loading": "Loading...",
                    "update_image_description": "Deploy a new version of your application by providing a new Docker image URL.",
                    "confirm_update_image": "Are you sure? Updating the image for '{name}' will take a few moments.",
                    "update_image_button": "Update image",
                    "update_image_button_loading": "Updating...",
                    "rebuild_description": "Rebuild your project from the latest GitHub repository code. This will pull the latest changes and redeploy your application.",
                    "confirm_rebuild": "Are you sure you want to rebuild the project '{name}'? This may take a few moments.",
                    "rebuild_button": "Rebuild from GitHub",
                    "rebuild_button_loading": "Rebuilding...",
                    "participants_list_label": "Participants:",
                    "manage_participants_title": "Manage participants",
                    "no_participants": "This project has no participants.",
                    "remove_participant_button": "Remove",
                    "confirm_remove_participant": "Are you sure you want to remove {name} from the project?",
                    "add_participant_label": "Add a participant (login)",
                    "add_participant_placeholder": "situ62394",
                    "add_participant_button": "Add",
                    "add_participant_button_loading": "Adding...",
                    "env_vars_description": "Changes will trigger a project restart to take effect. Values are encrypted at rest.",
                    "env_vars_updated_success": "Environment variables updated successfully. The project is restarting.",
                    "save_and_restart_button": "Save & Restart",
                    "save_and_restart_button_loading": "Saving...",
                    "persistent_volume_label": "Persistent Volume"
                },
                "database": {
                    "title": "Database",
                    "create_button": "Create Database",
                    "dashboard_title": "Database Dashboard",
                    "connection_info_title": "Connection Information",
                    "host": "Host",
                    "port": "Port",
                    "db_name": "Database Name",
                    "username": "Username",
                    "password": "Password",
                    "open_phpmyadmin": "Open phpMyAdmin",
                    "link_to_project_title": "Link to a Project",
                    "no_projects_to_link": "You have no projects available to link this database to.",
                    "select_project": "Select a project...",
                    "link_button": "Link to Project",
                    "unlink_button": "Unlink from Project",
                    "delete_button": "Delete Database",
                    "confirm_delete": "Are you sure you want to permanently delete this database? This action is irreversible.",
                    "no_db_linked": "No database is linked to this project.",
                    "unlinked_db_found": "You have an existing unlinked database ('{name}').",
                    "link_this_db_button": "Link this database",
                    "create_and_link_button": "Create & Link a New Database"
                },
                "admin": {
                    "title": "Admin dashboard",
                    "all_projects_title": "All projects",
                    "global_metrics_title": "Global metrics"
                },
                "errors": {
                    "PROJECT_NAME_TAKEN": "This project name is already taken.",
                    "OWNER_ALREADY_EXISTS": "You already own a project. Only one is allowed per user.",
                    "INVALID_PROJECT_NAME": "The project name is invalid. Use only letters, numbers, and hyphens.",
                    "INVALID_IMAGE_URL": "The provided Docker image URL is invalid or contains forbidden characters.",
                    "IMAGE_SCAN_FAILED": "Security scan failed: vulnerabilities were found in the image.",
                    "CLIENT_ERROR": "An unexpected client-side error occurred. Please try again.",
                    "DELETE_FAILED": "Failed to delete the project.",
                    "HTTP_ERROR_500": "An internal server error occurred. Please try again later or contact support.",
                    "UNAUTHORIZED": "Your session has expired. Please log in again.",
                    "OWNER_CANNOT_BE_PARTICIPANT": "The project owner cannot be added as a participant.",
                    "GITHUB_ACCOUNT_NOT_LINKED": "Your GitHub account is not linked. You must link it to deploy from a repository.",
                    "GITHUB_REPO_NOT_ACCESSIBLE": "The Hangar App does not have access to this repository. Please update your installation permissions. Then try again.",
                    "GITHUB_PACKAGE_NOT_PUBLIC": "Direct deployment from ghcr.io failed. Please ensure your package is set to 'Public'.",
                    "DEFAULT": "An unexpected error occurred. Please contact an administrator.",
                    "DATABASE_ALREADY_EXISTS": "You already own a database. Only one is allowed per user.",
                    "LINK_FAILED": "Failed to link the database to the project.",
                    "NOT_FOUND": "The requested resource was not found."
                }
            }"#,
        ),
        (
            "fr",
            r#"{
                "common": {
                    "loading": "Chargement...",
                    "error": "Une erreur est survenue.",
                    "owner": "Propriétaire",
                    "source_url": "URL Source",
                    "deployed_image": "Image Déployée",
                    "status": "Statut",
                    "created_on": "Créé le : {date}",
                    "back_to_home": "Retour à l'accueil",
                    "status_running": "En cours",
                    "status_exited": "Terminé",
                    "status_stopped": "Arrêté",
                    "status_dead": "Mort",
                    "status_restarting": "Redémarrage",
                    "status_created": "Créé",
                    "status_paused": "En pause",
                    "status_unknown": "Inconnu"
                },
                "home": {
                    "title": "Bienvenue sur Hangar",
                    "login_button": "Connexion avec Moodle",
                    "description": "Déployez et gérez facilement vos applications."
                },
                "nav": {
                    "home": "Accueil",
                    "admin": "Admin",
                    "logout": "Déconnexion"
                },
                 "footer": {
                    "about": "À propos",
                    "terms": "Conditions d'utilisation",
                    "privacy": "Politique de confidentialité",
                    "contact": "Contact"
                },
                "about": {
                    "title": "À propos de Hangar",
                    "hero_subtitle": "La plateforme de déploiement de Garage Isep",
                    "what_is_title": "Qu'est-ce que Hangar ?",
                    "what_is_p1": "Hangar est une plateforme de déploiement d'applications développée par Garage Isep. Elle permet aux étudiants de l'ISEP de déployer et gérer facilement leurs applications web sans se soucier de l'infrastructure technique.",
                    "what_is_p2": "Que vous souhaitiez héberger votre projet d'APP Web, votre projet de fin d'année ou autre chose, Hangar vous offre tous les outils nécessaires en quelques clics.",
                    
                    "features_title": "Fonctionnalités principales",
                    "feature_deploy_title": "Déploiement simplifié",
                    "feature_deploy_desc": "Connectez votre dépôt GitHub ou utilisez une image Docker. Hangar s'occupe du reste : build, scan de sécurité, déploiement automatique.",
                    "feature_database_title": "Bases de données MariaDB",
                    "feature_database_desc": "Créez et gérez vos bases de données en un clic.",
                    "feature_monitoring_title": "Monitoring temps réel",
                    "feature_monitoring_desc": "Suivez l'état de vos applications, consultez les logs et surveillez l'utilisation des ressources (CPU, RAM) via des tableaux de bord intuitifs.",
                    "feature_collab_title": "Collaboration",
                    "feature_collab_desc": "Ajoutez des participants à vos projets pour travailler en équipe.",
                    "feature_security_title": "Sécurité",
                    "feature_security_desc": "Scan automatique des vulnérabilités, chiffrement des secrets, isolation des conteneurs, HTTPS automatique avec Let's Encrypt.",
                    "feature_updates_title": "Mises à jour zero-downtime",
                    "feature_updates_desc": "Déployez de nouvelles versions sans interruption de service grâce au système de déploiement blue-green.",
                    
                    "tech_title": "Technologies utilisées",
                    "tech_intro": "Hangar est entièrement développé en Rust pour garantir performance, sécurité et fiabilité :",
                    "tech_frontend": "Frontend : Yew (WebAssembly)",
                    "tech_backend": "Backend : Axum (API REST)",
                    "tech_container": "Conteneurisation : Docker + Traefik",
                    "tech_db": "Bases de données : PostgreSQL + MariaDB",
                    
                    "team_title": "L'équipe",
                    "team_p1": "Hangar est développé par Simon TULOUP et maintenu par l'équipe DSI de Garage Isep.",
                    "team_p2": "Ce projet est nouveau. Si vous avez des retours à nous faire, n'hésitez pas à nous contacter !",
                    
                    "mission_title": "Notre mission",
                    "mission_p1": "Rendre le déploiement d'applications accessible à tous les étudiants, quel que soit leur niveau technique.",
                    "mission_p2": "Fournir une infrastructure robuste, moderne et sécurisée pour vous aider"
                },
                "terms": {
                    "title": "Conditions d'Utilisation",
                    "last_updated": "Dernière mise à jour : Octobre 2025",
                    "intro": "En utilisant Hangar, vous acceptez les conditions suivantes. Veuillez les lire attentivement.",
                    
                    "h2_1": "1. Acceptation des conditions",
                    "p1_1": "En accédant à Hangar et en déployant des applications, vous acceptez d'être lié par ces conditions d'utilisation. Si vous n'acceptez pas ces conditions, vous ne devez pas utiliser la plateforme.",
                    "p1_2": "Ces conditions peuvent être modifiées à tout moment. Les modifications prendront effet immédiatement après leur publication sur cette page.",
                    
                    "h2_2": "2. Éligibilité",
                    "p2_1": "Hangar est réservé exclusivement aux étudiants de l'Isep.",
                    "p2_2": "Vous devez disposer d'identifiants Isep valides pour accéder à la plateforme.",
                    
                    "h2_3": "3. Responsabilité de l'utilisateur",
                    "p3_1": "Vous êtes seul responsable du contenu que vous déployez sur Hangar. Vous vous engagez à :",
                    "p3_list_1": "Ne pas héberger de contenu illégal, diffamatoire, haineux ou portant atteinte aux droits d'autrui",
                    "p3_list_2": "Ne pas déployer de code malveillant, de virus, ou de tout élément nuisible",
                    "p3_list_3": "Ne pas utiliser la plateforme pour des activités commerciales sans autorisation",
                    "p3_list_4": "Respecter les droits de propriété intellectuelle (licences logicielles, droits d'auteur)",
                    "p3_list_5": "Ne pas compromettre la sécurité ou la stabilité de la plateforme",
                    "p3_2": "Garage Isep se réserve le droit de supprimer sans préavis tout projet ne respectant pas ces règles, et de suspendre ou bannir le compte utilisateur concerné.",
                    
                    "h2_4": "4. Limitation des ressources",
                    "p4_1": "Pour garantir une qualité de service équitable pour tous, chaque utilisateur est soumis aux limitations suivantes :",
                    "p4_list_1": "1 projet maximum par utilisateur",
                    "p4_list_2": "1 base de données maximum par utilisateur",
                    "p4_list_3": "512 MB de RAM par conteneur",
                    "p4_list_4": "50% d'un cœur CPU par conteneur",
                    "p4_list_5": "Stockage raisonnable pour les volumes persistants",
                    "p4_2": "Tout abus ou utilisation excessive des ressources (minage de cryptomonnaies, attaques par force brute, etc.) peut entraîner la suspension immédiate du projet sans préavis.",
                    
                    "h2_5": "5. Disponibilité du service",
                    "p5_1": "Garage Isep s'efforce de maintenir Hangar disponible 24h/24 et 7j/7, mais ne peut garantir une disponibilité à 100%.",
                    "p5_2": "Des maintenances programmées peuvent entraîner des interruptions temporaires. Nous nous efforçons de les annoncer à l'avance.",
                    "p5_3": "Garage Isep ne peut être tenu responsable des pertes de données, interruptions de service ou tout dommage résultant de l'utilisation de Hangar.",
                    
                    "h2_6": "6. Sauvegarde des données",
                    "p6_1": "Bien que nous effectuions des sauvegardes régulières de l'infrastructure, vous êtes responsable de la sauvegarde de vos propres données (code, base de données, fichiers).",
                    "p6_2": "Garage Isep ne peut être tenu responsable de la perte de données résultant d'une erreur, d'une défaillance technique ou de la suppression d'un projet.",
                    
                    "h2_7": "7. Confidentialité et sécurité",
                    "p7_1": "Vos identifiants de connexion sont gérés via le système CAS de l'Isep. Nous ne stockons pas vos mots de passe.",
                    "p7_2": "Les variables d'environnement et mots de passe de bases de données sont chiffrés en base de données (AES-256-GCM).",
                    "p7_3": "Les logs d'accès sont conservés pour des raisons de sécurité et de débogage.",
                    "p7_4": "Vous êtes responsable de la sécurité de vos applications déployées. Hangar fournit des outils (scan de vulnérabilités, isolation), mais ne peut garantir la sécurité de votre code.",
                    
                    "h2_8": "8. Propriété intellectuelle",
                    "p8_1": "Vous conservez tous les droits sur le code et le contenu que vous déployez sur Hangar.",
                    "p8_2": "En utilisant Hangar, vous accordez à Garage Isep une licence non-exclusive pour héberger, exécuter et distribuer votre contenu dans le cadre du service.",
                    "p8_3": "Le code source de Hangar est open-source et disponible sous licence appropriée.",
                    
                    "h2_9": "9. Résiliation",
                    "p9_1": "Vous pouvez supprimer votre projet à tout moment depuis l'interface utilisateur.",
                    "p9_2": "Garage Isep se réserve le droit de suspendre ou supprimer tout projet en cas de violation de ces conditions.",
                    "p9_3": "En cas de fin d'études ou de départ de l'Isep, votre compte pourra être désactivé après un délai de grâce.",
                    
                    "h2_10": "10. Contact et support",
                    "p10_1": "Pour toute question concernant ces conditions d'utilisation, contactez-nous à dsi@garageisep.com",
                    "p10_2": "Le support technique est fourni dans la mesure du possible, sans garantie de délai de réponse.",
                    
                    "acceptance": "En utilisant Hangar, vous confirmez avoir lu, compris et accepté ces conditions d'utilisation."
                },
                "privacy": {
                    "title": "Politique de confidentialité – Hangar",
                    "last_updated": "Dernière mise à jour : Décembre 2025",
                    "intro": "La présente politique de confidentialité informe les utilisateurs de Hangar sur la collecte, l'utilisation et la protection de leurs données conformément au RGPD et à la loi Informatique et Libertés.",

                    "h2_1": "1. Responsable du traitement",
                    "p1": "Garage Isep\nAssociation loi 1901 – R.N.A : W751249825\nSiège social : 28 Rue Notre Dame des Champs, 75006 Paris, France\nEmail : dsi@garageisep.com",

                    "h2_2": "2. Données personnelles collectées",
                    "p2_intro": "Les données suivantes peuvent être collectées :",
                    "p2_id": "• Données d’identification : prénom, email institutionnel ISEP, identifiant utilisateur",
                    "p2_tech": "• Données techniques : IP, logs de connexion, informations projet, logs applicatifs",
                    "p2_service": "• Données liées aux services : variables d’environnement, identifiants de bases de données chiffrés, ressources utilisées",
                    "p2_note": "Aucun mot de passe utilisateur n’est stocké par Hangar (authentification CAS).",

                    "h2_3": "3. Finalités du traitement",
                    "p3_1": "Fournir et gérer l’accès à Hangar",
                    "p3_2": "Permettre le déploiement et l’hébergement des applications",
                    "p3_3": "Assurer la sécurité et prévenir les abus",
                    "p3_4": "Surveiller performances et stabilité",
                    "p3_5": "Support technique et maintenance",
                    "p3_6": "Respect des obligations légales",

                    "h2_4": "4. Base légale du traitement",
                    "p4_1": "Exécution d’un service (article 6.1.b RGPD)",
                    "p4_2": "Intérêt légitime (article 6.1.f RGPD)",
                    "p4_3": "Obligations légales (article 6.1.c RGPD)",

                    "h2_5": "5. Durée de conservation des données",
                    "p5": "Données de compte : durée d’utilisation du service\nLogs : 12 mois maximum\nDonnées projet : supprimées à la suppression du projet",

                    "h2_6": "6. Destinataires des données",
                    "p6": "Membres habilités de l’équipe DSI et administrateurs techniques strictement nécessaires. Les données ne sont ni vendues ni cédées.",

                    "h2_7": "7. Hébergement des données",
                    "p7": "Hébergement en France / UE. Mesures techniques et organisationnelles pour garantir sécurité et confidentialité.",

                    "h2_8": "8. Sécurité des données",
                    "p8": "Chiffrement AES-256-GCM, accès restreint, isolation des conteneurs, HTTPS, sauvegardes régulières. Aucun système n’est totalement exempt de risques.",

                    "h2_9": "9. Droits des utilisateurs",
                    "p9": "Droit d’accès, rectification, effacement, limitation, opposition, portabilité. Contact : dsi@garageisep.com, réponse sous 30 jours.",

                    "h2_10": "10. Suppression du compte",
                    "p10": "Suppression possible via interface ou email, entraîne suppression des projets et données associées.",

                    "h2_11": "11. Cookies et traceurs",
                    "p11": "Utilisation uniquement de cookies strictement nécessaires. Aucun cookie publicitaire ou de traçage commercial.",

                    "h2_12": "12. Modification de la politique",
                    "p12": "Garage Isep peut modifier cette politique à tout moment. Les utilisateurs sont informés via la plateforme.",

                    "h2_13": "13. Réclamation CNIL",
                    "p13": "Commission Nationale de l’Informatique et des Libertés (CNIL) – www.cnil.fr",

                    "contact": "Contact : dsi@garageisep.com"
                    },
                "contact": {
                    "title": "Contact",
                    "p1": "Pour toute question, problème ou suggestion d'amélioration, n'hésitez pas à contacter l'équipe de Garage Isep.",
                    "p2_prefix": "Vous pouvez nous joindre par email à l'adresse : "
                },
                "auth": {
                    "logging_in": "Connexion en cours, veuillez patienter...",
                    "login_failed": "L'authentification a échoué. Veuillez réessayer.",
                    "ticket_missing": "Le ticket d'authentification est manquant. Veuillez retenter la connexion."
                },
                "dashboard": {
                    "welcome": "Bienvenue, {name} !",
                    "description": "Votre centre de déploiement d'applications.",
                    "create_project_button": "Nouveau projet ou BDD",
                    "owned_projects_title": "Mes Projets & Bases de Données",
                    "participating_projects_title": "Mes participations",
                    "empty_state_owned": "Vous n'avez encore aucun projet ni base de données.",
                    "empty_state_participating": "Vous ne participez à aucun projet."
                },
                "create_project": {
                    "title": "Créer un nouveau projet",
                    "documentation": "Veuillez consulter la ",
                    "github_tab": "Déployer depuis GitHub",
                    "direct_tab": "Avancé : Déployer depuis une image",
                    "description_github": "La méthode la plus simple. Votre code sera automatiquement build et déployé depuis un dépôt GitHub public.",
                    "description_direct": "Pour les utilisateurs avancés. Déployez un service à partir d'une image Docker **publique**. Note : les images privées de `ghcr.io` ne sont pas supportées dans ce mode.",
                    "name_label": "Nom du projet",
                    "name_placeholder": "mon-app-geniale",
                    "name_help": "Sera utilisé pour l'URL (ex: mon-app-geniale.garageisep.com). Lettres, chiffres et tirets uniquement.",
                    "github_repo_url_label": "URL du dépôt GitHub",
                    "github_repo_url_placeholder": "https://github.com/user/mon-projet",
                    "image_label": "URL de l'image Docker",
                    "image_placeholder": "mon-registre/mon-image:1.0",
                    "participants_label": "Participants",
                    "participants_placeholder": "situ62394, john.doe",
                    "participants_help": "Logins MOODLE des utilisateurs, séparés par des virgules. Ils auront un accès au projet.",
                    "submit_button": "Déployer le projet",
                    "submit_button_loading": "Déploiement en cours...",
                    "link_github_prompt": "Pour déployer depuis GitHub, vous devez d'abord installer l'application GitHub Hangar sur votre compte ou organisation.",
                    "link_github_button": "Installer l'application GitHub",
                    "github_connected_success": "Application GitHub connectée avec succès ! Vous pouvez maintenant réessayer de déployer votre projet.",
                    "volume_path_label": "Chemin du volume persistant (facultatif)",
                    "volume_path_help": "Chemin à l'intérieur du conteneur à persister.",
                    "env_vars_label": "Variables d'environnement (facultatif)",
                    "env_vars_help": "Une variable par ligne, au format KEY=VALUE.",
                    "database_tab": "Base de données seule",
                    "description_database": "Créez une base de données MariaDB autonome, sans la lier à un projet.",
                    "create_db_checkbox": "Créer et lier également une nouvelle base de données",
                    "github_branch_help": "Laissez vide pour utiliser la branche par défaut.",
                    "github_branch_label": "Branche GitHub (facultatif)",
                    "github_root_dir_label": "Dossier Racine (facultatif)",
                    "github_root_dir_help": "Chemin relatif vers le dossier contenant index.php. Ex: /src/public. Laissez vide si index.php est à la racine."
                },
                "project_dashboard": {
                    "title": "Tableau de bord du projet",
                    "visit_app_button": "Visiter l'application",
                    "card_title_info": "Informations du projet",
                    "card_title_controls": "Contrôles",
                    "card_title_logs": "Logs",
                    "card_title_metrics": "Métriques (en %)",
                    "card_title_danger": "Zone de danger",
                    "card_title_update_image": "Mettre à jour l'image",
                    "card_title_rebuild": "Reconstruire depuis GitHub",
                    "card_title_env_vars": "Gérer les Variables d'Environnement",
                    "logs_placeholder": "Cliquez sur 'Récupérer les logs' pour afficher les logs du conteneur",
                    "logs_empty": "Aucune sortie de log. Le conteneur n'écrit peut-être rien sur stdout/stderr, ou il est simplement silencieux.",
                    "logs_error": "Erreur lors de la récupération des logs : {error}",
                    "delete_button": "Supprimer le projet",
                    "confirm_delete": "Êtes-vous sûr de vouloir supprimer définitivement le projet '{name}' ? Cette action est irréversible.",
                    "confirm_delete_db_warning": "La base de données liée sera également supprimée définitiveement.",
                    "access_error_title": "Erreur d'accès",
                    "load_error_message": "Impossible de charger le projet : {error}",
                    "start_button": "Démarrer",
                    "stop_button": "Arrêter",
                    "restart_button": "Redémarrer",
                    "start_success": "Projet démarré avec succès !",
                    "stop_success": "Projet arrêté avec succès !",
                    "restart_success": "Projet redémarré avec succès !",
                    "fetch_logs_button": "Récupérer les logs",
                    "fetch_logs_loading": "Chargement...",
                    "update_image_description": "Déployez une nouvelle version de votre application en fournissant une nouvelle URL d'image Docker.",
                    "confirm_update_image": "Êtes-vous sûr ? La mise à jour de l'image pour '{name}' prendra quelques instants.",
                    "update_image_button": "Mettre à jour l'image",
                    "update_image_button_loading": "Mise à jour...",
                    "rebuild_description": "Reconstruisez votre projet à partir du dernier code du dépôt GitHub. Cela récupérera les dernières modifications et redéployera votre application.",
                    "confirm_rebuild": "Êtes-vous sûr de vouloir reconstruire le projet '{name}' ? Cela peut prendre quelques instants.",
                    "rebuild_button": "Reconstruire depuis GitHub",
                    "rebuild_button_loading": "Reconstruction en cours...",
                    "participants_list_label": "Participants :",
                    "manage_participants_title": "Gérer les participants",
                    "no_participants": "Ce projet n'a aucun participant.",
                    "remove_participant_button": "Retirer",
                    "confirm_remove_participant": "Êtes-vous sûr de vouloir retirer {name} du projet ?",
                    "add_participant_label": "Ajouter un participant (login)",
                    "add_participant_placeholder": "situ62394",
                    "add_participant_button": "Ajouter",
                    "add_participant_button_loading": "Ajout en cours...",
                    "env_vars_description": "Les changements entraîneront un redémarrage du projet pour être pris en compte. Les valeurs sont chiffrées au repos.",
                    "env_vars_updated_success": "Variables d'environnement mises à jour. Le projet est en cours de redémarrage.",
                    "save_and_restart_button": "Sauvegarder & Redémarrer",
                    "save_and_restart_button_loading": "Sauvegarde...",
                    "persistent_volume_label": "Volume Persistant"
                },
                "database": {
                    "title": "Base de Données",
                    "create_button": "Créer la base de données",
                    "dashboard_title": "Tableau de Bord de la Base de Données",
                    "connection_info_title": "Informations de Connexion",
                    "host": "Hôte",
                    "port": "Port",
                    "db_name": "Nom de la base",
                    "username": "Utilisateur",
                    "password": "Mot de passe",
                    "open_phpmyadmin": "Ouvrir phpMyAdmin",
                    "link_to_project_title": "Lier à un Projet",
                    "no_projects_to_link": "Vous n'avez aucun projet disponible pour lier cette base de données.",
                    "select_project": "Sélectionnez un projet...",
                    "link_button": "Lier au projet",
                    "unlink_button": "Délier du projet",
                    "delete_button": "Supprimer la base de données",
                    "confirm_delete": "Êtes-vous sûr de vouloir supprimer définitivement cette base de données ? Cette action est irréversible.",
                    "no_db_linked": "Aucune base de données n'est liée à ce projet.",
                    "unlinked_db_found": "Vous avez une base de données existante non liée ('{name}').",
                    "link_this_db_button": "Lier cette base de données",
                    "create_and_link_button": "Créer & Lier une nouvelle BDD"
                },
                "admin": {
                    "title": "Tableau de bord admin",
                    "all_projects_title": "Tous les projets",
                    "global_metrics_title": "Métriques globales"
                },
                "errors": {
                    "PROJECT_NAME_TAKEN": "Ce nom de projet est déjà utilisé.",
                    "OWNER_ALREADY_EXISTS": "Vous possédez déjà un projet. Un seul projet par utilisateur est autorisé.",
                    "INVALID_PROJECT_NAME": "Le nom du projet est invalide. Utilisez uniquement des lettres, des chiffres et des tirets.",
                    "INVALID_IMAGE_URL": "L'URL de l'image Docker est invalide ou contient des caractères interdits.",
                    "IMAGE_SCAN_FAILED": "L'analyse de sécurité a échoué : des vulnérabilités ont été trouvées dans l'image.",
                    "CLIENT_ERROR": "Une erreur inattendue est survenue côté client. Veuillez réessayer.",
                    "DELETE_FAILED": "La suppression du projet a échoué.",
                    "HTTP_ERROR_500": "Une erreur interne est survenue sur le serveur. Veuillez réessayer plus tard ou contacter le support.",
                    "UNAUTHORIZED": "Votre session a expiré. Veuillez vous reconnecter.",
                    "OWNER_CANNOT_BE_PARTICIPANT": "Le propriétaire du projet ne peut pas être ajouté comme participant.",
                    "GITHUB_ACCOUNT_NOT_LINKED": "Votre compte GitHub n'est pas lié. Vous devez le lier pour pouvoir déployer depuis un dépôt.",
                    "GITHUB_REPO_NOT_ACCESSIBLE": "L'application Hangar n'a pas accès à ce dépôt. Veuillez mettre à jour les permissions de votre installation. Puis réessayez.",
                    "GITHUB_PACKAGE_NOT_PUBLIC": "Le déploiement direct depuis ghcr.io a échoué. Veuillez vous assurer que votre paquet est bien en mode 'Public'.",
                    "DEFAULT": "Une erreur inattendue est survenue. Veuillez contacter un administrateur.",
                    "DATABASE_ALREADY_EXISTS": "Vous possédez déjà une base de données. Une seule est autorisée par utilisateur.",
                    "LINK_FAILED": "La liaison de la base de données au projet a échoué.",
                    "NOT_FOUND": "La ressource demandée n'a pas été trouvée."
                }
            }
            "#,
        ),
    ]);

    let default_language = window()
        .and_then(|w| w.navigator().language())
        .map(|lang| 
        {
            if lang.starts_with("fr") 
            {
                "fr".to_string()
            } 
            else 
            {
                "en".to_string()
            }
        })
        .unwrap_or_else(|| "en".to_string());

    let config = I18nProviderConfig 
    {
        translations,
        default_language,
        ..Default::default()
    };

    html! 
    {
        <I18nProvider ..config>
            <UserProvider>
                <BrowserRouter>
                    <div style="display: flex; flex-direction: column; min-height: 100vh;">
                        <Nav />
                        <main style="flex-grow: 1;">
                            <Switch<AppRoute> render={switch} />
                        </main>
                        <Footer />
                    </div>
                </BrowserRouter>
            </UserProvider>
        </I18nProvider>
    }
}