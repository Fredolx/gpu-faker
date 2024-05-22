import { Component, OnInit } from '@angular/core';
import { NgbModal } from '@ng-bootstrap/ng-bootstrap';
import { invoke } from '@tauri-apps/api'
import { ToastrService } from 'ngx-toastr';
import { take } from 'rxjs';
import { ErrorModalComponent } from '../error-modal/error-modal.component';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss'
})
export class HomeComponent implements OnInit {
  processing: boolean = false;
  currentGPU: string = "Unknown";
  desiredGPU: string = "";

  constructor(private toast: ToastrService, private modal: NgbModal) {

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
    let error = e as string;
    console.error(error);
    this.toast.error("An error occured. Click here for more info")
      .onTap
      .pipe(take(1))
      .subscribe(() => this.showError(error));  
  }

  showError(error: string) {
    const modalRef = this.modal.open(ErrorModalComponent, { backdrop: 'static', size: 'xl'});
    modalRef.componentInstance.name = 'ErrorModal';
    modalRef.componentInstance.error = error;
  }
}
