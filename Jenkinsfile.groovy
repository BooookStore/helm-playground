pipeline {
    agent any

    stages {
        stage('Stage 1') {
            steps {
                echo 'Hello world!'
                sh 'ls -l'
            }
        }
        stage('Stage 2') {
            steps {
                echo 'Hello world!'
            }
        }
    }
}