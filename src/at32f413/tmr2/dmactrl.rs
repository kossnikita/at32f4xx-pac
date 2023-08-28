#[doc = "Register `DMACTRL` reader"]
pub type R = crate::R<DMACTRL_SPEC>;
#[doc = "Register `DMACTRL` writer"]
pub type W = crate::W<DMACTRL_SPEC>;
#[doc = "Field `ADDR` reader - DMA transfer address offset"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - DMA transfer address offset"]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DTB` reader - DMA transfer bytes"]
pub type DTB_R = crate::FieldReader;
#[doc = "Field `DTB` writer - DMA transfer bytes"]
pub type DTB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - DMA transfer address offset"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA transfer bytes"]
    #[inline(always)]
    pub fn dtb(&self) -> DTB_R {
        DTB_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA transfer address offset"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<DMACTRL_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Bits 8:12 - DMA transfer bytes"]
    #[inline(always)]
    #[must_use]
    pub fn dtb(&mut self) -> DTB_W<DMACTRL_SPEC, 8> {
        DTB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTRL_SPEC;
impl crate::RegisterSpec for DMACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactrl::R`](R) reader structure"]
impl crate::Readable for DMACTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmactrl::W`](W) writer structure"]
impl crate::Writable for DMACTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACTRL to value 0"]
impl crate::Resettable for DMACTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
