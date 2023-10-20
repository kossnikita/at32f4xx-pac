#[doc = "Register `HPTXSTS` reader"]
pub type R = crate::R<HPTXSTS_SPEC>;
#[doc = "Register `HPTXSTS` writer"]
pub type W = crate::W<HPTXSTS_SPEC>;
#[doc = "Field `PTXFSPCAVAIL` reader - Periodic transmit data FIFO space available"]
pub type PTXFSPCAVAIL_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFSPCAVAIL` writer - Periodic transmit data FIFO space available"]
pub type PTXFSPCAVAIL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PTXQSPCAVAIL` reader - Periodic transmit request queue space available"]
pub type PTXQSPCAVAIL_R = crate::FieldReader;
#[doc = "Field `PTXQTOP` reader - Top of the periodic transmit request queue"]
pub type PTXQTOP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Periodic transmit data FIFO space available"]
    #[inline(always)]
    pub fn ptxfspcavail(&self) -> PTXFSPCAVAIL_R {
        PTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic transmit request queue space available"]
    #[inline(always)]
    pub fn ptxqspcavail(&self) -> PTXQSPCAVAIL_R {
        PTXQSPCAVAIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the periodic transmit request queue"]
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXSTS")
            .field(
                "ptxfspcavail",
                &format_args!("{}", self.ptxfspcavail().bits()),
            )
            .field(
                "ptxqspcavail",
                &format_args!("{}", self.ptxqspcavail().bits()),
            )
            .field("ptxqtop", &format_args!("{}", self.ptxqtop().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HPTXSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Periodic transmit data FIFO space available"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfspcavail(&mut self) -> PTXFSPCAVAIL_W<HPTXSTS_SPEC> {
        PTXFSPCAVAIL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTGFS_Host periodic transmit FIFO/queue status register (OTGFS_HPTXSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPTXSTS_SPEC;
impl crate::RegisterSpec for HPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxsts::R`](R) reader structure"]
impl crate::Readable for HPTXSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hptxsts::W`](W) writer structure"]
impl crate::Writable for HPTXSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPTXSTS to value 0x0008_0100"]
impl crate::Resettable for HPTXSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0100;
}
