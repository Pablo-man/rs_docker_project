# Hello world whit rust
## Results
<p align="center">
    <img src="./public/img/result.png" alt="Hello from rust">
</p>

Aplicacion web que despliega un servidor web y levanta una simple API con una ruta que muestra el mensaje **¡hello world with go!**

## :open_book: How to use
### Pre-requisites
   * lenguaje rust
   * editor de codigo
---

* Clone the repository
    ```
    git clone https://github.com/Pablo-man/java_docker_project.git
    ```
* Abre el programa con el editor de codigo de tu preferencia
* Abre una terminal que apunte a la raiz del proyecto
* Ejecuta el comando:

    `go run main.go`
* Visita tu `localhost:8080` para visualizar los resultados

    > [!TIP]
    > Por defecto la aplicacion se desplegara en el puerto `8080`, si es necesario cambiarlo a otro debe modificarlo desde el archivo `main.go` y en el apartado `http.ListenAndServe(":<PORTNUMBER>", nil)` colocar el puerto deseado

## :rocket: How to run with docker
Visitar el siguiente enlance para conocer el proceso de generacion de la imagen del proyecto

### Como crear la imagen
```html
docker build -t <NEWIMAGENAME> .
<!-- Example -->
<!-- docker build -t rusthello . -->
```
### Como ejecutar la imagen (Contenedor)
* Descargar del repositorio remoto la imagen

    `docker pull pamendeza/rust_docker_project `
* Crear el contenedor a partir de la imagen
    ```html
    docker run -d --name <NEWDOCKERNAME> -p <PORT>:8080 <IMAGENAME>

    <!-- Example -->
    <!-- docker run -d --name gohello -p 3000:8080 pamendeza/run_docker_project:v1.0 -->
    ```
    > [!TIP]
    > Si se desea mapear a un puerto diferente la aplicacion, unicamente se debe cambiar el numero de puerto que se encuentra a la izquierda debido a que el de la derecha es propio del contenedor y no se puede modificar.
### Como subir al repositorio remoto la imagen
* Colocar un tag a la imagen, en la cual se especifica el nombre o id de la imagen que deseamos subir seguido de el nombre de usuario de dockerhub, nombre del repositorio y version de la imagen
    ```html
    docker tag <IMAGENAME>||<IDIMAGE> <USERNAME>/<REPOSITORIENAME>:<VERSION>
    <!-- Example -->
    <!-- docker tag rusthello pamendeza/rust_docker_project:v1.0 -->
    ```
* Subir la imagen tageada la cual consiste del nombre de usuario de dockerhub, el nombre del repositorio y la version de la imagen. Hacia el repositorio remoto
    ```html
    docker push  <USERNAME>/<REPOSITORIENAME>:<VERSION>
    <!-- Example -->
    <!-- docker push pamendeza/rust_docker_project:v1.0 -->
    ```
    [View results](#results)
## :light_rail: PAAS Deploy (Railway)