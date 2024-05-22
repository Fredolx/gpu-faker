import { Component, OnInit } from '@angular/core';
import { invoke } from '@tauri-apps/api'
import { ToastrService } from 'ngx-toastr';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss'
})
export class HomeComponent implements OnInit {
  processing: boolean = false;
  currentGPU: string = "Unknown";
  desiredGPU: string = "";
  
  constructor(private toast: ToastrService) {

  }

  ngOnInit(): void {
    this.getCurrentGPU();
  }

  getCurrentGPU() {
    invoke('getCurrentGPU')
      .then((response) => this.currentGPU = (response as string))
      .catch((_) => this.toast.error("Failed to fetch GPU. You may be running an unsupported OS"))
  }

  applyDesiredGPU() {
    invoke('applyDesiredGPU')
      .then(_ => _)
      .catch(_ => this.toast.error("Failed to apply desired GPU. You may be running an unsupported OS"))
  }
}
