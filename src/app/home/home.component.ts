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
    invoke('get_current_gpu')
      .then((response) => this.currentGPU = (response as string))
      .catch((e) => {
        this.handleError(e);
      });
  }

  applyDesiredGPU() {
    this.processing = true;
    invoke('apply_desired_gpu', { gpu: this.desiredGPU })
      .then(_ => this.getCurrentGPU())
      .catch(e => {
        this.handleError(e);
      })
      .finally(() => this.processing = false);
  }

  handleError(e: any) {
    console.log(e as string);
    this.toast.error("Failed to fetch GPU. You may be running an unsupported OS")
  }
}
