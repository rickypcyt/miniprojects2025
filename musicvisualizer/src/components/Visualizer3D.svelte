<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import * as THREE from 'three';
  import type { AudioInput } from './AudioInput';
  export let audio: AudioInput;
  let container: HTMLDivElement;
  let renderer: THREE.WebGLRenderer;
  let scene: THREE.Scene;
  let camera: THREE.PerspectiveCamera;
  let animationId: number;
  let mesh: THREE.Mesh;

  function animate() {
    animationId = requestAnimationFrame(animate);
    const freq = audio.getFrequencyData();
    if (freq && mesh) {
      // Ejemplo: deformar geometría con el audio
      const positions = (mesh.geometry as THREE.SphereGeometry).attributes.position;
      for (let i = 0; i < positions.count; i++) {
        const amp = freq[i % freq.length] / 255;
        positions.setZ(i, positions.getZ(i) + (Math.random()-0.5) * amp * 2);
      }
      positions.needsUpdate = true;
      mesh.material.color.setHSL(Math.random(), 1, 0.5);
    }
    mesh.rotation.x += 0.01;
    mesh.rotation.y += 0.02;
    renderer.render(scene, camera);
  }

  onMount(() => {
    renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(container.offsetWidth, container.offsetHeight);
    container.appendChild(renderer.domElement);
    scene = new THREE.Scene();
    camera = new THREE.PerspectiveCamera(75, container.offsetWidth / container.offsetHeight, 0.1, 1000);
    camera.position.z = 5;
    // Geometría trippy
    const geometry = new THREE.SphereGeometry(1, 128, 128);
    const material = new THREE.MeshStandardMaterial({ color: 0x00ffcc, wireframe: true });
    mesh = new THREE.Mesh(geometry, material);
    scene.add(mesh);
    // Luces techno
    const light1 = new THREE.PointLight(0xff00ff, 2, 100);
    light1.position.set(5, 5, 5);
    scene.add(light1);
    const light2 = new THREE.PointLight(0x00ffff, 2, 100);
    light2.position.set(-5, -5, 5);
    scene.add(light2);
    animate();
    window.addEventListener('resize', onResize);
  });

  function onResize() {
    if (!renderer || !camera) return;
    renderer.setSize(container.offsetWidth, container.offsetHeight);
    camera.aspect = container.offsetWidth / container.offsetHeight;
    camera.updateProjectionMatrix();
  }

  onDestroy(() => {
    cancelAnimationFrame(animationId);
    renderer.dispose();
    window.removeEventListener('resize', onResize);
  });
</script>

<div bind:this={container} class="w-full h-full absolute top-0 left-0"></div> 