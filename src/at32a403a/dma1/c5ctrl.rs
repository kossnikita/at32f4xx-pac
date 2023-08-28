#[doc = "Register `C5CTRL` reader"]
pub type R = crate::R<C5CTRL_SPEC>;
#[doc = "Register `C5CTRL` writer"]
pub type W = crate::W<C5CTRL_SPEC>;
#[doc = "Field `CHEN` reader - Channel enable"]
pub type CHEN_R = crate::BitReader;
#[doc = "Field `CHEN` writer - Channel enable"]
pub type CHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FDTIEN` reader - Full data transfer interrupt enable"]
pub type FDTIEN_R = crate::BitReader;
#[doc = "Field `FDTIEN` writer - Full data transfer interrupt enable"]
pub type FDTIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDTIEN` reader - Half data transfer interrupt enable"]
pub type HDTIEN_R = crate::BitReader;
#[doc = "Field `HDTIEN` writer - Half data transfer interrupt enable"]
pub type HDTIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTERRIEN` reader - Data transfer error interrupt enable"]
pub type DTERRIEN_R = crate::BitReader;
#[doc = "Field `DTERRIEN` writer - Data transfer error interrupt enable"]
pub type DTERRIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTD` reader - Data transfer direction"]
pub type DTD_R = crate::BitReader;
#[doc = "Field `DTD` writer - Data transfer direction"]
pub type DTD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LM` reader - Loop mode"]
pub type LM_R = crate::BitReader;
#[doc = "Field `LM` writer - Loop mode"]
pub type LM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PINCM` reader - Peripheral address increment mode"]
pub type PINCM_R = crate::BitReader;
#[doc = "Field `PINCM` writer - Peripheral address increment mode"]
pub type PINCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MINCM` reader - Memory address increment mode"]
pub type MINCM_R = crate::BitReader;
#[doc = "Field `MINCM` writer - Memory address increment mode"]
pub type MINCM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWIDTH` reader - Peripheral data bit width"]
pub type PWIDTH_R = crate::FieldReader;
#[doc = "Field `PWIDTH` writer - Peripheral data bit width"]
pub type PWIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MWIDTH` reader - Memory data bit width"]
pub type MWIDTH_R = crate::FieldReader;
#[doc = "Field `MWIDTH` writer - Memory data bit width"]
pub type MWIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CHPL` reader - Channel Priority level"]
pub type CHPL_R = crate::FieldReader;
#[doc = "Field `CHPL` writer - Channel Priority level"]
pub type CHPL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `M2M` reader - Memory to memory mode"]
pub type M2M_R = crate::BitReader;
#[doc = "Field `M2M` writer - Memory to memory mode"]
pub type M2M_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full data transfer interrupt enable"]
    #[inline(always)]
    pub fn fdtien(&self) -> FDTIEN_R {
        FDTIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half data transfer interrupt enable"]
    #[inline(always)]
    pub fn hdtien(&self) -> HDTIEN_R {
        HDTIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data transfer error interrupt enable"]
    #[inline(always)]
    pub fn dterrien(&self) -> DTERRIEN_R {
        DTERRIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    pub fn dtd(&self) -> DTD_R {
        DTD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Loop mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral address increment mode"]
    #[inline(always)]
    pub fn pincm(&self) -> PINCM_R {
        PINCM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Memory address increment mode"]
    #[inline(always)]
    pub fn mincm(&self) -> MINCM_R {
        MINCM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Peripheral data bit width"]
    #[inline(always)]
    pub fn pwidth(&self) -> PWIDTH_R {
        PWIDTH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Memory data bit width"]
    #[inline(always)]
    pub fn mwidth(&self) -> MWIDTH_R {
        MWIDTH_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel Priority level"]
    #[inline(always)]
    pub fn chpl(&self) -> CHPL_R {
        CHPL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    pub fn m2m(&self) -> M2M_R {
        M2M_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<C5CTRL_SPEC, 0> {
        CHEN_W::new(self)
    }
    #[doc = "Bit 1 - Full data transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdtien(&mut self) -> FDTIEN_W<C5CTRL_SPEC, 1> {
        FDTIEN_W::new(self)
    }
    #[doc = "Bit 2 - Half data transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdtien(&mut self) -> HDTIEN_W<C5CTRL_SPEC, 2> {
        HDTIEN_W::new(self)
    }
    #[doc = "Bit 3 - Data transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dterrien(&mut self) -> DTERRIEN_W<C5CTRL_SPEC, 3> {
        DTERRIEN_W::new(self)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dtd(&mut self) -> DTD_W<C5CTRL_SPEC, 4> {
        DTD_W::new(self)
    }
    #[doc = "Bit 5 - Loop mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<C5CTRL_SPEC, 5> {
        LM_W::new(self)
    }
    #[doc = "Bit 6 - Peripheral address increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pincm(&mut self) -> PINCM_W<C5CTRL_SPEC, 6> {
        PINCM_W::new(self)
    }
    #[doc = "Bit 7 - Memory address increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn mincm(&mut self) -> MINCM_W<C5CTRL_SPEC, 7> {
        MINCM_W::new(self)
    }
    #[doc = "Bits 8:9 - Peripheral data bit width"]
    #[inline(always)]
    #[must_use]
    pub fn pwidth(&mut self) -> PWIDTH_W<C5CTRL_SPEC, 8> {
        PWIDTH_W::new(self)
    }
    #[doc = "Bits 10:11 - Memory data bit width"]
    #[inline(always)]
    #[must_use]
    pub fn mwidth(&mut self) -> MWIDTH_W<C5CTRL_SPEC, 10> {
        MWIDTH_W::new(self)
    }
    #[doc = "Bits 12:13 - Channel Priority level"]
    #[inline(always)]
    #[must_use]
    pub fn chpl(&mut self) -> CHPL_W<C5CTRL_SPEC, 12> {
        CHPL_W::new(self)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn m2m(&mut self) -> M2M_W<C5CTRL_SPEC, 14> {
        M2M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C5CTRL_SPEC;
impl crate::RegisterSpec for C5CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c5ctrl::R`](R) reader structure"]
impl crate::Readable for C5CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c5ctrl::W`](W) writer structure"]
impl crate::Writable for C5CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C5CTRL to value 0"]
impl crate::Resettable for C5CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
